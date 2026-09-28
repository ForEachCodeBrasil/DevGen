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
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, PhysicalPosition, Position, State,
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
    let result = state.registry.generate(&req.generator_id, req.options)?;

    // Record history
    if let Some(text) = &result.text {
        let entry = format!("{}: {}", req.generator_id, text);
        let _ = state.store.update(|p| {
            p.history.insert(0, entry);
            if p.history.len() > 30 {
                p.history.truncate(30);
            }
        });
    }

    Ok(result)
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
        "quick.copy_nick" => ("nick", serde_json::json!({})),
        "quick.copy_random_number" => ("random_number", serde_json::json!({ "length": 8 })),
        "quick.copy_name" => ("name", serde_json::json!({})),
        _ => {
            return Err(GeneratorError::InvalidOptions(format!(
                "Unknown action: {}",
                action
            )))
        }
    };

    let result = state.registry.generate(generator_id, options)?;

    // Record history
    if let Some(text) = &result.text {
        let entry = format!("{}: {}", generator_id, text);
        let _ = state.store.update(|p| {
            p.history.insert(0, entry);
            if p.history.len() > 30 {
                p.history.truncate(30);
            }
        });
    }

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
fn exit_app(app: tauri::AppHandle) {
    app.exit(0);
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

        if !quick.is_empty() {
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
                    "quick.copy_nick" => "Nick / Apelido",
                    "quick.copy_random_number" => "Número Aleatório",
                    "quick.copy_name" => "Nome de Pessoa",
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

fn show_main_window(app: &tauri::AppHandle, tray_click_position: Option<PhysicalPosition<f64>>) {
    if let Some(window) = app.get_webview_window("main") {
        if let Some(position) = tray_click_position {
            if let Ok(size) = window.outer_size() {
                let x = position.x - (size.width as f64 / 2.0);
                let y = position.y + 8.0;
                let _ = window.set_position(Position::Physical(PhysicalPosition::new(
                    x.round() as i32,
                    y.round() as i32,
                )));
            }
        }

        let _ = window.show();
        let _ = window.set_focus();
    }
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
    registry.register(crate::generators::impls::CurriculumGenerator);
    registry.register(crate::generators::impls::NameGenerator);
    registry.register(crate::generators::impls::CompanyGenerator);
    registry.register(crate::generators::impls::BankAccountGenerator);
    registry.register(crate::generators::impls::VehicleGenerator);
    registry.register(crate::generators::impls::VehiclePlateGenerator);
    registry.register(crate::generators::impls::NickGenerator);
    registry.register(crate::generators::impls::CreditCardGenerator);
    registry.register(crate::generators::impls::PasswordGenerator);
    registry.register(crate::generators::impls::UuidGenerator);
    registry.register(crate::generators::impls::RandomNumberGenerator);
    registry.register(crate::generators::impls::LoremIpsumGenerator);
    registry.register(crate::generators::impls::LoremPixelGenerator);
    registry.register(crate::generators::impls::QrCodeGenerator);
    registry.register(crate::generators::impls::MetaTagsGenerator);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_generators,
            generate,
            quick_generate,
            get_preferences,
            save_preferences,
            exit_app,
            rebuild_tray_quick_menu
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let store = Arc::new(Store::new(app.app_handle()));
            app.manage(AppState { registry, store });

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_always_on_top(true);
            }

            let show_i = MenuItem::with_id(app, "show", "Abrir DevGen", true, None::<&str>)?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;

            // Default tray quick actions; user choices rebuild via rebuild_tray_quick_menu.
            let uuid_item =
                MenuItem::with_id(app, "quick.copy_uuid", "UUID v4", true, None::<&str>)?;
            let password_item = MenuItem::with_id(
                app,
                "quick.copy_password",
                "Senha Segura",
                true,
                None::<&str>,
            )?;
            let lorem_item = MenuItem::with_id(
                app,
                "quick.copy_lorem_ipsum",
                "Lorem Ipsum",
                true,
                None::<&str>,
            )?;
            let nick_item =
                MenuItem::with_id(app, "quick.copy_nick", "Nick / Apelido", true, None::<&str>)?;
            let random_item = MenuItem::with_id(
                app,
                "quick.copy_random_number",
                "Número Aleatório",
                true,
                None::<&str>,
            )?;
            let name_item =
                MenuItem::with_id(app, "quick.copy_name", "Nome de Pessoa", true, None::<&str>)?;

            let generate_submenu = Submenu::with_items(
                app,
                "Gerar...",
                true,
                &[
                    &uuid_item,
                    &password_item,
                    &lorem_item,
                    &nick_item,
                    &random_item,
                    &name_item,
                ],
            )?;

            let menu = Menu::with_items(app, &[&show_i, &sep1, &generate_submenu, &sep2, &quit_i])?;

            let resource_path = app
                .path()
                .resolve("icons/tray-icon.png", tauri::path::BaseDirectory::Resource)
                .unwrap();

            let img = ::image::open(&resource_path).expect("Failed to load tray icon");
            let rgba_img = img.into_rgba8();
            let (width, height) = rgba_img.dimensions();
            let tray_icon = tauri::image::Image::new_owned(rgba_img.into_raw(), width, height);

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(tray_icon)
                .icon_as_template(false)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app, None),
                    "quit" => app.exit(0),
                    id if id.starts_with("quick.") => {
                        let action_id = id.to_string();
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = app_handle.emit("quick-action", action_id);
                        });
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Down,
                        position,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle(), Some(position));
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let app_handle = app.app_handle().clone();
                window.on_window_event(move |event| match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        if let Some(w) = app_handle.get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                    tauri::WindowEvent::Focused(false) => {
                        if let Some(w) = app_handle.get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                    _ => {}
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
