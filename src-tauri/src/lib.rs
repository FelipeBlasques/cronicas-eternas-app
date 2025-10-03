// src-tauri/src/lib.rs

#[cfg(not(mobile))]
pub fn run() {
    use tauri::{Manager, Emitter}; // <-- Emitter é necessário para .emit()

    // prepara registro do esquema para o instalador (macOS/Windows/Linux)
    tauri_plugin_deep_link::prepare("com.phenomen.flc");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // registra o esquema "myflc" (troque o nome se quiser outro)
            tauri_plugin_deep_link::register("myflc", move |request| {
                // ex.: "myflc://join?token=ABC&world=42"
                // emite o evento para o frontend
                let _ = app_handle.emit("deep-link", request);
            })?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
