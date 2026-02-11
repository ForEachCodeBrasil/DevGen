mod generators;

use generators::{
    registry::GeneratorRegistry, AppPreferences, GenerateRequest, GenerateResponse,
    GeneratorDefinition, GeneratorError, QuickGenerateResponse,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};

struct AppState {
    registry: GeneratorRegistry,
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
    _state: State<'_, AppState>,
    _action: String,
) -> Result<QuickGenerateResponse, GeneratorError> {
    // Stub implementation
    Ok(QuickGenerateResponse {
        text: "Quick generated text".to_string(),
    })
}

#[tauri::command]
fn get_preferences() -> AppPreferences {
    // Stub implementation
    AppPreferences {
        locale: "pt-BR".to_string(),
    }
}

#[tauri::command]
fn save_preferences(_prefs: AppPreferences) -> Result<(), String> {
    // Stub implementation
    Ok(())
}

#[tauri::command]
fn rebuild_tray_quick_menu(_quick: Vec<String>) -> Result<(), String> {
    // Stub implementation
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = GeneratorRegistry::new();
    // TODO: Register actual generators here later
    // registry.register(MyGenerator::new());

    tauri::Builder::default()
        .manage(AppState { registry })
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
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
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
