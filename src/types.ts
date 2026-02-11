export type GeneratorCategory =
    | "Documents"
    | "Person"
    | "Company"
    | "Vehicle"
    | "Utils";

export interface GeneratorDefinition {
    id: string;
    name: string;
    category: GeneratorCategory;
    description: string;
}

export interface GenerateRequest {
    generator_id: string;
    options: Record<string, any>;
}

export interface GenerateResponse {
    text?: string;
    metadata?: Record<string, any>;
}

export interface GeneratorError {
    message: string;
}

export interface AppPreferences {
    locale: string;
}

export interface QuickGenerateResponse {
    text: string;
}
