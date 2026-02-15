use crate::store::Store;
use chrono::{DateTime, Duration, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const LEMON_ACTIVATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/activate";
const LEMON_VALIDATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/validate";
const LEMON_DEACTIVATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/deactivate";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccessTier {
    Free,
    Pro,
}

impl AccessTier {
    pub fn is_pro(&self) -> bool {
        matches!(self, Self::Pro)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LicenseStatus {
    Active,
    Inactive,
    Expired,
    Disabled,
    Invalid,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseState {
    pub tier: AccessTier,
    pub status: LicenseStatus,
    pub license_key: Option<String>,
    pub instance_id: Option<String>,
    pub customer_email: Option<String>,
    pub last_validated_at: Option<String>,
}

impl Default for LicenseState {
    fn default() -> Self {
        Self {
            tier: AccessTier::Free,
            status: LicenseStatus::Inactive,
            license_key: None,
            instance_id: None,
            customer_email: None,
            last_validated_at: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LicenseManager {
    client: Client,
    checkout_url: String,
    store_id: Option<u64>,
    product_id: Option<u64>,
    variant_id: Option<u64>,
    instance_name: String,
    grace_days: i64,
}

#[derive(Debug, Deserialize)]
struct LemonMeta {
    store_id: u64,
    product_id: u64,
    variant_id: u64,
    customer_email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LemonActivation {
    activated: bool,
    instance: LemonInstance,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    meta: Option<LemonMeta>,
}

#[derive(Debug, Deserialize)]
struct LemonInstance {
    id: String,
}

#[derive(Debug, Deserialize)]
struct LemonValidation {
    valid: bool,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    license_key: Option<LemonLicenseKey>,
    #[serde(default)]
    meta: Option<LemonMeta>,
}

#[derive(Debug, Deserialize)]
struct LemonLicenseKey {
    status: String,
}

#[derive(Debug, Serialize)]
struct ActivatePayload<'a> {
    license_key: &'a str,
    instance_name: &'a str,
}

#[derive(Debug, Serialize)]
struct ValidatePayload<'a> {
    license_key: &'a str,
    instance_id: &'a str,
}

#[derive(Debug, Serialize)]
struct DeactivatePayload<'a> {
    license_key: &'a str,
    instance_id: &'a str,
}

impl LicenseManager {
    pub fn from_env() -> Self {
        let checkout_url = std::env::var("DEVGEN_LEMON_CHECKOUT_URL")
            .unwrap_or_else(|_| "https://lemonsqueezy.com".to_string());
        let store_id = std::env::var("DEVGEN_LEMON_STORE_ID")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());
        let product_id = std::env::var("DEVGEN_LEMON_PRODUCT_ID")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());
        let variant_id = std::env::var("DEVGEN_LEMON_VARIANT_ID")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());
        let instance_name = std::env::var("DEVGEN_LEMON_INSTANCE_NAME")
            .unwrap_or_else(|_| "devgen-desktop".to_string());
        let grace_days = std::env::var("DEVGEN_LEMON_GRACE_DAYS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(7);

        Self {
            client: Client::new(),
            checkout_url,
            store_id,
            product_id,
            variant_id,
            instance_name,
            grace_days,
        }
    }

    pub fn checkout_url(&self) -> String {
        self.checkout_url.clone()
    }

    pub fn resolve_access_tier(&self, store: &Store) -> LicenseState {
        let Ok(prefs) = store.get() else {
            return LicenseState::default();
        };

        let mut state = prefs.license_state;
        if state.tier.is_pro() {
            if let Some(last) = &state.last_validated_at {
                if let Ok(parsed) = DateTime::parse_from_rfc3339(last) {
                    let last_utc = parsed.with_timezone(&Utc);
                    let grace_deadline = last_utc + Duration::days(self.grace_days);
                    if Utc::now() > grace_deadline {
                        state.tier = AccessTier::Free;
                        state.status = LicenseStatus::Expired;
                    }
                }
            }
        }

        state
    }

    pub fn activate(
        &self,
        store: &Store,
        key: &str,
        email: Option<&str>,
    ) -> Result<LicenseState, String> {
        let payload = ActivatePayload {
            license_key: key,
            instance_name: &self.instance_name,
        };

        let response = self
            .client
            .post(LEMON_ACTIVATE_URL)
            .json(&payload)
            .send()
            .map_err(|e| format!("Failed to activate license: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "License activation failed with status {}",
                response.status()
            ));
        }

        let data: LemonActivation = response
            .json()
            .map_err(|e| format!("Failed to decode activation response: {}", e))?;

        if !data.activated {
            return Err(data
                .error
                .unwrap_or_else(|| "License key was not activated".to_string()));
        }

        if let Some(meta) = &data.meta {
            self.assert_meta(meta)?;
        }

        let mut next = LicenseState {
            tier: AccessTier::Pro,
            status: LicenseStatus::Active,
            license_key: Some(key.to_string()),
            instance_id: Some(data.instance.id),
            customer_email: email
                .map(|v| v.to_string())
                .or_else(|| data.meta.and_then(|m| m.customer_email)),
            last_validated_at: Some(Utc::now().to_rfc3339()),
        };

        store.update(|p| {
            p.license_state = next.clone();
        })?;

        next = self.resolve_access_tier(store);
        Ok(next)
    }

    pub fn validate(&self, store: &Store) -> Result<LicenseState, String> {
        let prefs = store.get()?;
        let current = prefs.license_state;

        let Some(key) = current.license_key.clone() else {
            let next = LicenseState::default();
            store.update(|p| p.license_state = next.clone())?;
            return Ok(next);
        };

        let Some(instance_id) = current.instance_id.clone() else {
            let next = LicenseState::default();
            store.update(|p| p.license_state = next.clone())?;
            return Ok(next);
        };

        let payload = ValidatePayload {
            license_key: &key,
            instance_id: &instance_id,
        };

        let response = self
            .client
            .post(LEMON_VALIDATE_URL)
            .json(&payload)
            .send()
            .map_err(|e| format!("Failed to validate license: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "License validation failed with status {}",
                response.status()
            ));
        }

        let data: LemonValidation = response
            .json()
            .map_err(|e| format!("Failed to decode validation response: {}", e))?;

        if let Some(meta) = &data.meta {
            self.assert_meta(meta)?;
        }

        let mut next = current;
        next.last_validated_at = Some(Utc::now().to_rfc3339());

        if data.valid {
            next.tier = AccessTier::Pro;
            next.status = LicenseStatus::Active;
            if let Some(meta) = data.meta {
                next.customer_email = meta.customer_email.or(next.customer_email);
            }
        } else {
            next.tier = AccessTier::Free;
            next.status = Self::status_from_error(data.error, data.license_key.as_ref());
        }

        store.update(|p| {
            p.license_state = next.clone();
        })?;

        Ok(next)
    }

    pub fn deactivate(&self, store: &Store) -> Result<(), String> {
        let prefs = store.get()?;
        let current = prefs.license_state;

        if let (Some(key), Some(instance_id)) =
            (current.license_key.as_ref(), current.instance_id.as_ref())
        {
            let payload = DeactivatePayload {
                license_key: key,
                instance_id,
            };

            let _ = self.client.post(LEMON_DEACTIVATE_URL).json(&payload).send();
        }

        store.update(|p| {
            p.license_state = LicenseState::default();
        })
    }

    fn assert_meta(&self, meta: &LemonMeta) -> Result<(), String> {
        if let Some(expected_store) = self.store_id {
            if meta.store_id != expected_store {
                return Err("License does not belong to configured store".to_string());
            }
        }

        if let Some(expected_product) = self.product_id {
            if meta.product_id != expected_product {
                return Err("License does not belong to configured product".to_string());
            }
        }

        if let Some(expected_variant) = self.variant_id {
            if meta.variant_id != expected_variant {
                return Err("License does not belong to configured variant".to_string());
            }
        }

        Ok(())
    }

    fn status_from_error(
        error: Option<String>,
        key_data: Option<&LemonLicenseKey>,
    ) -> LicenseStatus {
        if let Some(v) = key_data {
            return match v.status.as_str() {
                "active" => LicenseStatus::Active,
                "inactive" => LicenseStatus::Inactive,
                "expired" => LicenseStatus::Expired,
                "disabled" => LicenseStatus::Disabled,
                _ => LicenseStatus::Unknown,
            };
        }

        match error.unwrap_or_default().to_lowercase().as_str() {
            e if e.contains("expired") => LicenseStatus::Expired,
            e if e.contains("disabled") => LicenseStatus::Disabled,
            e if e.contains("invalid") => LicenseStatus::Invalid,
            _ => LicenseStatus::Unknown,
        }
    }
}
