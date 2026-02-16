use crate::store::Store;
use chrono::{DateTime, Duration, Utc};
use lycento_sdk::{
    ActivateOptions, LycentoClient, LycentoConfig, ValidateOptions, DeactivateOptions,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    client: Arc<LycentoClient>,
    checkout_url: String,
    grace_days: i64,
}

impl LicenseManager {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();

        let base_url = std::env::var("LYCENTO_BASE_URL")
            .unwrap_or_else(|_| "https://lycento.test".to_string());
        let api_key = std::env::var("LYCENTO_API_KEY").ok();
        let checkout_url = std::env::var("LYCENTO_CHECKOUT_URL")
            .unwrap_or_else(|_| format!("{}/checkout", base_url));
        let grace_days = std::env::var("LYCENTO_GRACE_DAYS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(7);

        let mut config = LycentoConfig::new(&base_url);
        if let Some(key) = api_key {
            config = config.with_api_key(&key);
        }

        let client = LycentoClient::new(config)
            .expect("Failed to create Lycento client");

        Self {
            client: Arc::new(client),
            checkout_url,
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

    pub async fn activate(
        &self,
        store: &Store,
        key: &str,
        email: Option<&str>,
    ) -> Result<LicenseState, String> {
        let options = ActivateOptions::new(key);

        let response = self.client.activate(options).await
            .map_err(|e| format!("Failed to activate license: {}", e.message()))?;

        let next = LicenseState {
            tier: AccessTier::Pro,
            status: LicenseStatus::Active,
            license_key: Some(response.license.key),
            instance_id: Some(response.activation.device_id),
            customer_email: email.map(|v| v.to_string()),
            last_validated_at: Some(Utc::now().to_rfc3339()),
        };

        store.update(|p| {
            p.license_state = next.clone();
        })?;

        let resolved = self.resolve_access_tier(store);
        Ok(resolved)
    }

    pub async fn validate(&self, store: &Store) -> Result<LicenseState, String> {
        let prefs = store.get()?;
        let current = prefs.license_state;

        let Some(key) = current.license_key.clone() else {
            let next = LicenseState::default();
            store.update(|p| p.license_state = next.clone())?;
            return Ok(next);
        };

        let Some(device_id) = current.instance_id.clone() else {
            let next = LicenseState::default();
            store.update(|p| p.license_state = next.clone())?;
            return Ok(next);
        };

        let options = ValidateOptions::new(&key).with_device_id(&device_id);

        let response = self.client.validate(options).await
            .map_err(|e| format!("Failed to validate license: {}", e.message()))?;

        let mut next = current;
        next.last_validated_at = Some(Utc::now().to_rfc3339());

        if response.valid {
            next.tier = AccessTier::Pro;
            next.status = LicenseStatus::Active;
        } else {
            next.tier = AccessTier::Free;
            next.status = Self::status_from_license(&response.license);
        }

        store.update(|p| {
            p.license_state = next.clone();
        })?;

        Ok(next)
    }

    pub async fn deactivate(&self, store: &Store) -> Result<(), String> {
        let prefs = store.get()?;
        let current = prefs.license_state;

        if let (Some(key), Some(device_id)) =
            (current.license_key.as_ref(), current.instance_id.as_ref())
        {
            let options = DeactivateOptions::new(key, device_id);
            let _ = self.client.deactivate(options).await;
        }

        store.update(|p| {
            p.license_state = LicenseState::default();
        })
    }

    fn status_from_license(license: &lycento_sdk::LicenseInfo) -> LicenseStatus {
        match license.status.as_str() {
            "active" => LicenseStatus::Active,
            "inactive" => LicenseStatus::Inactive,
            "expired" => LicenseStatus::Expired,
            "revoked" | "disabled" => LicenseStatus::Disabled,
            "invalid" => LicenseStatus::Invalid,
            _ => LicenseStatus::Unknown,
        }
    }
}
