// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_updater::UpdaterExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let builder = app.handle().updater_builder().build();

            tauri::async_runtime::spawn(async move {
                if let Ok(x) = builder {
                    if let Ok(Some(update)) = x.check().await {
                        update.download_and_install(|_,_|{}, ||{}).await.unwrap();
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
