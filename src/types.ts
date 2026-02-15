export type GeneratorCategory =
    | "documents"
    | "person"
    | "company"
    | "vehicle"
    | "utilities";

export interface GeneratorDefinition {
    id: string;
    name: string;
    category: GeneratorCategory;
    description: string;
    requires_pro: boolean;
    options?: Record<string, any>; // JSON Schema or similar
}

export interface GenerateRequest {
    generator_id: string;
    options: Record<string, any>;
}

export interface GenerateResponse {
    text?: string;
    metadata?: Record<string, any>;
    base64_artifact?: string;
}

export interface GeneratorError {
    message: string;
}

export interface AppPreferences {
    locale: string;
    quick_actions: string[];
    history: string[]; // TODO: Define specific HistoryItem struct later
    generator_last_options: Record<string, any>;
    license_state: LicenseState;
}

export interface QuickGenerateResponse {
    text: string;
}

export type AccessTier = 'free' | 'pro';

export type LicenseStatus =
    | 'active'
    | 'inactive'
    | 'expired'
    | 'disabled'
    | 'invalid'
    | 'unknown';

export interface LicenseState {
    tier: AccessTier;
    status: LicenseStatus;
    license_key?: string | null;
    instance_id?: string | null;
    customer_email?: string | null;
    last_validated_at?: string | null;
}
