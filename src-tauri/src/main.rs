
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::menus::{drMenu, drEvent};

fn main() {
//   let tray_menu1 = SystemTrayMenu::new()
//   .add_item(CustomMenuItem::new("toggle", "Toggle"))
//   .add_item(CustomMenuItem::new("new", "New window"))
//   .add_item(CustomMenuItem::new("icon_1", "Tray Icon 1"))
//   .add_item(CustomMenuItem::new("icon_2", "Tray Icon 2"))
//   .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
//   .add_item(CustomMenuItem::new("exit_app", "Quit"));
// let tray_menu2 = SystemTrayMenu::new()
//   .add_item(CustomMenuItem::new("toggle", "Toggle"))
//   .add_item(CustomMenuItem::new("new", "New window"))
//   .add_item(CustomMenuItem::new("switch_menu", "Switch Menu"))
//   .add_item(CustomMenuItem::new("exit_app", "Quit"));
// let is_menu1 = AtomicBool::new(true);



  let ctx = tauri::generate_context!();
  // let aboutMe = AboutMetadata::new();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![])
    .menu(menus::drMenu())
    .on_menu_event(menus::drEvent)
    .run(ctx)
    .expect("error while running tauri application");
}
