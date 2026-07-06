use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::{
    Code,
    GlobalShortcutExt,
    Modifiers,
    Shortcut,
    ShortcutState,
};
use tauri::Manager;
use arboard::Clipboard;
#[tauri::command]
fn get_clipboard_text() -> Result<String, String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| e.to_string())?;

    clipboard
        .get_text()
        .map_err(|e| e.to_string())
}
#[tauri::command]
fn set_clipboard_text(text: String) -> Result<(), String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| e.to_string())?;

    clipboard
        .set_text(text)
        .map_err(|e| e.to_string())
}
#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}
#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}
#[tauri::command]
async fn rewrite_text(
    text: String,
    mode: String,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let prompt = match mode.as_str() {
        "rewrite" => format!(
            "Rewrite while keeping the meaning:\n\n{}",
            text
        ),

        "professional" => format!(
            "Rewrite professionally while keeping the meaning:\n\n{}",
            text
        ),

        "friendly" => format!(
            "Rewrite in a friendly conversational tone:\n\n{}",
            text
        ),

        "summarize" => format!(
            "Summarize the following text in 1-2 concise sentences:\n\n{}",
            text
        ),

        "grammar" => format!(
            "Fix grammar and spelling only. Do not change the meaning:\n\n{}",
            text
        ),

        _ => format!(
            "Rewrite while keeping the meaning:\n\n{}",
            text
        ),
    };

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&OllamaRequest {
            model: "qwen2.5:14b-instruct".to_string(),
            prompt,
            stream: false,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result: OllamaResponse =
        response.json().await.map_err(|e| e.to_string())?;

    Ok(result.response.trim().to_string())
}
#[tauri::command]
fn ping() -> String {
    "GhostWrite Alive 🚀".to_string()
}
#[tauri::command]
fn hide_window(
    app: tauri::AppHandle,
) -> Result<(), String> {
    if let Some(window) =
        app.get_webview_window("main")
    {
        window
            .hide()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        println!("🔥 Shortcut Triggered!");

                        if shortcut.matches(
                            Modifiers::SHIFT | Modifiers::ALT,
                            Code::KeyG,
                        ) {
                            println!("🚀 Alt + Shift + G Pressed");
                            if let Some(window) = app.get_webview_window("main") {
                                let visible = window.is_visible().unwrap_or(false);

                                if visible {
                                    let _ = window.hide();
                                    println!("🙈 Window Hidden");
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    println!("👀 Window Shown");
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let shortcut = Shortcut::new(
                Some(Modifiers::SHIFT | Modifiers::ALT),
                Code::KeyG,
            );

            app.global_shortcut().register(shortcut)?;

            println!("✅ Shortcut Registered");

            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![
                ping,
                get_clipboard_text,
                set_clipboard_text,
                rewrite_text,
                hide_window
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}