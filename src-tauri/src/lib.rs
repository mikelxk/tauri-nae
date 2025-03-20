// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{ utils::config::parse, Manager, WebviewWindowBuilder};
use url::Url;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_url() -> String {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() > 1 {
        return args[1].clone();
    }
    return "https://xradventure.8thwall.app/studio-daf/".to_string();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            let main_window = app.get_webview_window("main").unwrap().hide();
            #[cfg(mobile)]
            let main_window = app.get_webview_window("main").unwrap();
            let url = get_url();
            let parsed_url = Url::parse(&url).unwrap_or_else(|e| {
                eprintln!("Error parsing URL: {}" , url);
                Url::parse("https://xradventure.8thwall.app/studio-daf/").unwrap()
            });
            WebviewWindowBuilder::new(
                app,
                "webview",
                tauri::WebviewUrl::External(parsed_url),
            )
            .build()?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_ios_fs::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
