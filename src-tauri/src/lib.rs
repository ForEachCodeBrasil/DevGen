pub mod datasets;
mod generators;
mod store;

use generators::{
    registry::GeneratorRegistry, AppPreferences, GenerateRequest, GenerateResponse,
    GeneratorDefinition, GeneratorError, QuickGenerateResponse,
};
use std::sync::Arc;
use store::Store;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State,
};

struct AppState {
    registry: GeneratorRegistry,
    store: Arc<Store>,
}

#[tauri::command]
fn list_generators(state: State<'_, AppState>) -> Vec<GeneratorDefinition> {
    state.registry.list()
}

#[tauri::command]
fn generate(
    state: State<'_, AppState>,
    req: GenerateRequest,
) -> Result<GenerateResponse, GeneratorError> {
    state.registry.generate(&req.generator_id, req.options)
}

#[tauri::command]
fn quick_generate(
    state: State<'_, AppState>,
    action: String,
) -> Result<QuickGenerateResponse, GeneratorError> {
    let (generator_id, options) = match action.as_str() {
        "quick.copy_cpf_masked" => ("cpf", serde_json::json!({ "mask": true })),
        "quick.copy_cpf_unmasked" => ("cpf", serde_json::json!({ "mask": false })),
        "quick.copy_cnpj_masked" => ("cnpj", serde_json::json!({ "mask": true })),
        "quick.copy_cnpj_unmasked" => ("cnpj", serde_json::json!({ "mask": false })),
        "quick.copy_rg_masked" => ("rg", serde_json::json!({ "mask": true })),
        "quick.copy_rg_unmasked" => ("rg", serde_json::json!({ "mask": false })),
        "quick.copy_person_full" => ("person", serde_json::json!({ "mask": true })),
        "quick.copy_company_full" => ("company", serde_json::json!({ "mask": true })),
        "quick.copy_vehicle_full" => ("vehicle", serde_json::json!({ "mask": true })),
        "quick.copy_credit_card" => (
            "credit_card",
            serde_json::json!({ "mask": true, "brand": "visa" }),
        ),
        "quick.copy_password" => (
            "password",
            serde_json::json!({ "length": 16, "uppercase": true, "lowercase": true, "numbers": true, "symbols": true }),
        ),
        "quick.copy_uuid" => ("uuid", serde_json::json!({ "hyphens": true })),
        "quick.copy_lorem_ipsum" => ("lorem_ipsum", serde_json::json!({ "paragraphs": 1 })),
        _ => {
            return Err(GeneratorError::InvalidOptions(format!(
                "Unknown action: {}",
                action
            )))
        }
    };

    let result = state.registry.generate(generator_id, options)?;

    Ok(QuickGenerateResponse {
        text: result.text.unwrap_or_default(),
    })
}

#[tauri::command]
fn get_preferences(state: State<'_, AppState>) -> Result<AppPreferences, String> {
    state.store.get()
}

#[tauri::command]
fn save_preferences(state: State<'_, AppState>, prefs: AppPreferences) -> Result<(), String> {
    state.store.update(|p| *p = prefs)
}

#[tauri::command]
fn rebuild_tray_quick_menu(app: tauri::AppHandle, quick: Vec<String>) -> Result<(), String> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder};

    // Attempt to retrieve the tray (assuming ID "tray")
    if let Some(tray) = app.tray_by_id("tray") {
        let menu = MenuBuilder::new(&app).build().map_err(|e| e.to_string())?;

        let show_i = MenuItemBuilder::with_id("show", "Abrir DevGen")
            .build(&app)
            .map_err(|e| e.to_string())?;
        let quit_i = MenuItemBuilder::with_id("quit", "Sair")
            .build(&app)
            .map_err(|e| e.to_string())?;

        menu.append(&show_i).map_err(|e| e.to_string())?;

        // Add separator
        // Note: PredefinedMenuItem::separator(&app) might be needed, using stub for now if complex

        // Add Quick Actions
        if !quick.is_empty() {
            // For each quick action, add a menu item
            for action_id in quick {
                let label = match action_id.as_str() {
                    "quick.copy_cpf_masked" => "CPF (Formatado)",
                    "quick.copy_cpf_unmasked" => "CPF (Números)",
                    "quick.copy_cnpj_masked" => "CNPJ (Formatado)",
                    "quick.copy_cnpj_unmasked" => "CNPJ (Números)",
                    "quick.copy_rg_masked" => "RG (Formatado)",
                    "quick.copy_person_full" => "Pessoa Completa",
                    "quick.copy_company_full" => "Empresa Completa",
                    "quick.copy_vehicle_full" => "Veículo Completo",
                    "quick.copy_credit_card" => "Cartão de Crédito",
                    "quick.copy_password" => "Senha Segura",
                    "quick.copy_uuid" => "UUID v4",
                    "quick.copy_lorem_ipsum" => "Lorem Ipsum",
                    // Fallback to ID if unknown
                    _ => action_id.as_str(),
                };

                let item = MenuItemBuilder::with_id(&action_id, label)
                    .build(&app)
                    .map_err(|e| e.to_string())?;
                menu.append(&item).map_err(|e| e.to_string())?;
            }
        }

        menu.append(&quit_i).map_err(|e| e.to_string())?;

        tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = GeneratorRegistry::new();

    // Register generators
    registry.register(crate::generators::impls::CpfGenerator);
    registry.register(crate::generators::impls::CnpjGenerator);
    registry.register(crate::generators::impls::CepGenerator);
    registry.register(crate::generators::impls::RgGenerator);
    registry.register(crate::generators::impls::CnhGenerator);
    registry.register(crate::generators::impls::PisGenerator);
    registry.register(crate::generators::impls::TituloEleitorGenerator);
    registry.register(crate::generators::impls::RenavamGenerator);
    registry.register(crate::generators::impls::CertidaoNascimentoGenerator);
    registry.register(crate::generators::impls::CertidaoCasamentoGenerator);
    registry.register(crate::generators::impls::CertidaoObitoGenerator);
    registry.register(crate::generators::impls::InscricaoEstadualGenerator);
    registry.register(crate::generators::impls::PersonGenerator);
    registry.register(crate::generators::impls::NameGenerator);
    registry.register(crate::generators::impls::CompanyGenerator);
    registry.register(crate::generators::impls::BankAccountGenerator);
    registry.register(crate::generators::impls::VehicleGenerator);
    registry.register(crate::generators::impls::VehiclePlateGenerator);
    registry.register(crate::generators::impls::CreditCardGenerator);
    registry.register(crate::generators::impls::PasswordGenerator);
    registry.register(crate::generators::impls::UuidGenerator);
    registry.register(crate::generators::impls::LoremIpsumGenerator);
    registry.register(crate::generators::impls::MetaTagsGenerator);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let store = Arc::new(Store::new(app.app_handle()));
            app.manage(AppState { registry, store });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_generators,
            generate,
            quick_generate,
            get_preferences,
            save_preferences,
            rebuild_tray_quick_menu
        ])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Abrir DevGen", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    action_id => {
                        // Assuming this is a quick action, emit it to the frontend
                        // The frontend will handle the generation and copying to clipboard
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("quick-action", action_id);
                        }
                    }
                })
                .on_tray_icon_event(|tray, event| if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event {
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            window.hide().unwrap();
            api.prevent_close();
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
