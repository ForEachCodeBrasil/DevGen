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
}

export interface QuickGenerateResponse {
    text: string;
}
