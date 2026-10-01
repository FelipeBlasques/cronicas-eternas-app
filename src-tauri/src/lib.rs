use tauri::webview::{NewWindowResponse, WebviewWindowBuilder};
use tauri::{AppHandle, Manager, Url, WebviewUrl};

// Crônicas Eternas: aplicativo do grupo, baseado no FLC (Foundry Lightweight Client)
// de Phenomen, licença MIT. A janela principal abre o portal; quando o portal manda
// o jogador para o Foundry, o jogo abre numa janela própria e o portal continua aberto.

/// Endereço do portal, que abre na janela principal
const PORTAL: &str = "https://cronicaseternas.com";

/// Endereço do Foundry: navegações para cá abrem na janela do jogo
const HOST_FOUNDRY: &str = "vtt.cronicaseternas.com";

#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {e}"))
}

#[tauri::command]
fn write_text_file(path: String, contents: String) -> Result<(), String> {
    std::fs::write(&path, contents).map_err(|e| format!("Failed to write file: {e}"))
}

#[tauri::command]
async fn open_webview(
    app: AppHandle,
    url: String,
    id: String,
    title: String,
    incognito: bool,
) -> Result<(), String> {
    // Sanitize ID to remove non-alphanumeric characters
    let sanitized_id: String = id.chars().filter(|c| c.is_alphanumeric()).collect();
    let mut new_id = format!("foundry{}", sanitized_id);

    // Check if a window with this label already exists
    if app.webview_windows().contains_key(&new_id) {
        let random_number = rand::random::<u32>() % 1000000;
        new_id = format!("foundry{}{}", sanitized_id, random_number);
    }

    WebviewWindowBuilder::new(
        &app,
        &new_id,
        tauri::WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?),
    )
    .title(format!("Foundry VTT - {}", title))
    .center()
    .closable(true)
    .devtools(true)
    .disable_drag_drop_handler()
    .focused(true)
    .general_autofill_enabled(false)
    .incognito(incognito)
    .inner_size(1280.0, 800.0)
    .maximizable(true)
    .minimizable(true)
    .resizable(true)
    .zoom_hotkeys_enabled(true)
    .on_new_window(|_url, _features| {
        // Allow popup windows to open
        NewWindowResponse::Allow
    })
    .build()
    .map_err(|e| format!("Failed to create webview: {}", e))?;

    Ok(())
}

/// Abre o Foundry na janela do jogo. Se ela já existir, só troca o endereço e traz para a frente.
fn abrir_jogo(app: &AppHandle, url: Url) {
    if let Some(janela) = app.get_webview_window("jogo") {
        let _ = janela.navigate(url);
        let _ = janela.unminimize();
        let _ = janela.set_focus();
        return;
    }

    let resultado = WebviewWindowBuilder::new(app, "jogo", WebviewUrl::External(url))
        .title("Crônicas Eternas: Foundry")
        .inner_size(1280.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .maximized(true)
        .focused(true)
        .devtools(true)
        .disable_drag_drop_handler()
        .general_autofill_enabled(false)
        .zoom_hotkeys_enabled(true)
        .on_new_window(|_url, _features| {
            // O Foundry abre janelas extras (fichas destacadas, por exemplo)
            NewWindowResponse::Allow
        })
        .build();

    if let Err(erro) = resultado {
        eprintln!("Não foi possível abrir a janela do jogo: {erro}");
    }
}

/// Cria a janela principal já no portal
fn criar_janela_principal(app: &AppHandle) -> tauri::Result<()> {
    let portal: Url = PORTAL.parse().expect("endereço do portal inválido");
    let handle = app.clone();

    WebviewWindowBuilder::new(app, "main", WebviewUrl::External(portal))
        .title("Crônicas Eternas")
        .inner_size(1280.0, 820.0)
        .min_inner_size(800.0, 600.0)
        .center()
        .disable_drag_drop_handler()
        .zoom_hotkeys_enabled(true)
        .on_navigation(move |url| {
            if url.host_str() == Some(HOST_FOUNDRY) {
                let handle = handle.clone();
                let url = url.clone();
                // Cria a janela fora deste evento, para não travar no Windows
                tauri::async_runtime::spawn(async move { abrir_jogo(&handle, url) });
                return false; // o portal continua na janela principal
            }
            true
        })
        .build()?;

    Ok(())
}

#[cfg(not(mobile))]
pub fn run() {
    #[cfg(target_os = "windows")]
    unsafe {
        std::env::set_var(
            "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
            "--force-high-performance-gpu --allow-insecure-localhost --allow-running-insecure-content --block-new-web-contents=false",
        );
    }

    // WebKitGTK white screen bug workaround
    // https://github.com/khoj-ai/pipali/pull/44
    #[cfg(target_os = "linux")]
    unsafe {
        for var in [
            "WEBKIT_DISABLE_DMABUF_RENDERER",
            "WEBKIT_DISABLE_COMPOSITING_MODE",
        ] {
            if std::env::var_os(var).is_none() {
                std::env::set_var(var, "1");
            }
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            open_webview,
            read_text_file,
            write_text_file
        ])
        .setup(|app| {
            criar_janela_principal(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
