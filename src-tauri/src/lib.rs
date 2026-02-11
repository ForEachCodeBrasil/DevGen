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
    Manager, State,
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
    _state: State<'_, AppState>,
    _action: String,
) -> Result<QuickGenerateResponse, GeneratorError> {
    // Stub implementation
    Ok(QuickGenerateResponse {
        text: "Quick generated text".to_string(),
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
                // ideally we'd look up the name, here we just use the ID as label for MVP
                let item = MenuItemBuilder::with_id(&action_id, &action_id)
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
