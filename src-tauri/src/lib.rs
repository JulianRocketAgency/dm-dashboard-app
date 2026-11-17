use std::time::Duration;

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

fn notify(app: &AppHandle, title: &str, msg: &str) {
    app.dialog()
        .message(msg.to_string())
        .title(title.to_string())
        .show(|_| {});
}

/// Controleert op een nieuwe versie. Bij een update vraagt de app of je nu wilt updaten (1 klik).
async fn check_for_updates(app: AppHandle, manual: bool) {
    let updater = match app.updater() {
        Ok(u) => u,
        Err(e) => {
            if manual {
                notify(&app, "Updates", &format!("Controleren is niet gelukt: {e}"));
            }
            return;
        }
    };

    match updater.check().await {
        Ok(Some(update)) => {
            let msg = format!(
                "Er is een nieuwe versie beschikbaar: {} (je gebruikt {}).\n\nNu updaten? De app start daarna automatisch opnieuw op.",
                update.version, update.current_version
            );
            let a = app.clone();
            let accepted = tauri::async_runtime::spawn_blocking(move || {
                a.dialog()
                    .message(msg)
                    .title("Update beschikbaar")
                    .buttons(MessageDialogButtons::OkCancelCustom(
                        "Nu updaten".to_string(),
                        "Later".to_string(),
                    ))
                    .blocking_show()
            })
            .await
            .unwrap_or(false);

            if accepted {
                match update.download_and_install(|_, _| {}, || {}).await {
                    Ok(_) => app.restart(),
                    Err(e) => notify(&app, "Update mislukt", &format!("{e}")),
                }
            }
        }
        Ok(None) => {
            if manual {
                notify(&app, "Geen updates", "Je gebruikt de nieuwste versie.");
            }
        }
        Err(e) => {
            if manual {
                notify(&app, "Updates", &format!("Controleren is niet gelukt: {e}"));
            }
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let h = app.handle();

            let check = MenuItem::with_id(h, "check_updates", "Controleer op updates…", true, None::<&str>)?;
            let reload = MenuItem::with_id(h, "reload", "Vernieuwen", true, Some("CmdOrCtrl+R"))?;

            let edit_menu = Submenu::with_items(
                h,
                "Bewerken",
                true,
                &[
                    &PredefinedMenuItem::undo(h, Some("Ongedaan maken"))?,
                    &PredefinedMenuItem::redo(h, Some("Opnieuw"))?,
                    &PredefinedMenuItem::separator(h)?,
                    &PredefinedMenuItem::cut(h, Some("Knip"))?,
                    &PredefinedMenuItem::copy(h, Some("Kopieer"))?,
                    &PredefinedMenuItem::paste(h, Some("Plak"))?,
                    &PredefinedMenuItem::select_all(h, Some("Selecteer alles"))?,
                ],
            )?;

            #[cfg(target_os = "macos")]
            let menu = {
                let app_menu = Submenu::with_items(
                    h,
                    "DM Dashboard",
                    true,
                    &[
                        &PredefinedMenuItem::about(h, Some("Over DM Dashboard"), None)?,
                        &check,
                        &PredefinedMenuItem::separator(h)?,
                        &PredefinedMenuItem::hide(h, Some("Verberg DM Dashboard"))?,
                        &PredefinedMenuItem::hide_others(h, Some("Verberg anderen"))?,
                        &PredefinedMenuItem::show_all(h, Some("Toon alles"))?,
                        &PredefinedMenuItem::separator(h)?,
                        &PredefinedMenuItem::quit(h, Some("Stop DM Dashboard"))?,
                    ],
                )?;
                let view_menu = Submenu::with_items(
                    h,
                    "Weergave",
                    true,
                    &[&reload, &PredefinedMenuItem::fullscreen(h, Some("Volledig scherm"))?],
                )?;
                let window_menu = Submenu::with_items(
                    h,
                    "Venster",
                    true,
                    &[
                        &PredefinedMenuItem::minimize(h, Some("Minimaliseer"))?,
                        &PredefinedMenuItem::close_window(h, Some("Sluit venster"))?,
                    ],
                )?;
                Menu::with_items(h, &[&app_menu, &edit_menu, &view_menu, &window_menu])?
            };

            #[cfg(not(target_os = "macos"))]
            let menu = {
                let file_menu = Submenu::with_items(
                    h,
                    "Bestand",
                    true,
                    &[
                        &check,
                        &PredefinedMenuItem::separator(h)?,
                        &PredefinedMenuItem::quit(h, Some("Afsluiten"))?,
                    ],
                )?;
                let view_menu = Submenu::with_items(h, "Weergave", true, &[&reload])?;
                Menu::with_items(h, &[&file_menu, &edit_menu, &view_menu])?
            };
            app.set_menu(menu)?;

            // Controleer kort na het starten en daarna elke 4 uur op updates.
            let handle = h.clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await;
                loop {
                    check_for_updates(handle.clone(), false).await;
                    tokio::time::sleep(Duration::from_secs(4 * 60 * 60)).await;
                }
            });
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "check_updates" => {
                let h = app.clone();
                tauri::async_runtime::spawn(async move {
                    check_for_updates(h, true).await;
                });
            }
            "reload" => {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.eval("window.location.reload()");
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("DM Dashboard kon niet starten");
}
