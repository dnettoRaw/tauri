#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use tauri::api::shell;
// use tauri::{
  // CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu,  WindowUrl,
// };

fn main() {
  let ctx = tauri::generate_context!();

  tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![])
    .run(ctx)
    .expect("error while running tauri application");
}
// .create_window("main", WindowUrl::default(), |win, webview| {
//   let win = win
//     .title("Tauri Template")
//     .resizable(true)
//     .decorations(true)
//     .always_on_top(false)
//     .inner_size(800.0, 550.0)
//     .min_inner_size(400.0, 200.0)
//     .skip_taskbar(false)
//     .fullscreen(false);
//   return (win, webview);
// })
// .menu(Menu::with_items([
//   #[cfg(target_os = "macos")]
//   MenuEntry::Submenu(Submenu::new(
//     &ctx.package_info().name,
//     Menu::with_items([
//       MenuItem::About(ctx.package_info().name.clone()).into(),
//       MenuItem::Separator.into(),
//       // MenuItem::Services.into(), for some raison if i ignore services comme empty for this tauri-build version
//       // MenuItem::Separator.into(),
//       MenuItem::Hide.into(),
//       MenuItem::HideOthers.into(),
//       MenuItem::ShowAll.into(),
//       MenuItem::Separator.into(),
//       MenuItem::Quit.into(),
//     ]),
//   )),
//   MenuEntry::Submenu(Submenu::new(
//     "File",
//     Menu::with_items([MenuItem::CloseWindow.into()]),
//   )),
//   MenuEntry::Submenu(Submenu::new(
//     "Edit",
//     Menu::with_items([
//       MenuItem::Undo.into(),
//       MenuItem::Redo.into(),
//       MenuItem::Separator.into(),
//       MenuItem::Cut.into(),
//       MenuItem::Copy.into(),
//       MenuItem::Paste.into(),
//       #[cfg(not(target_os = "macos"))]
//       MenuItem::Separator.into(),
//       MenuItem::SelectAll.into(),
//     ]),
//   )),
//   MenuEntry::Submenu(Submenu::new(
//     "View",
//     Menu::with_items([MenuItem::EnterFullScreen.into()]),
//   )),
//   MenuEntry::Submenu(Submenu::new(
//     "Window",
//     Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
//   )),
//   // You should always have a Help menu on macOS because it will automatically
//   // show a menu search field
//   MenuEntry::Submenu(Submenu::new(
//     "Help",
//     Menu::with_items([
//       CustomMenuItem::new("id:GitHub", "Learn More").into(),
//       CustomMenuItem::new("id:Tauri", "Tauri Doc").into()]),
//   )),
// ]))
// .on_menu_event(|event| {
//   let event_name = event.menu_item_id();
//   match event_name {
//     "id:GitHub" => {
//       let url = "https://github.com/dnettoRaw/tauri".to_string();
//       shell::open(&event.window().shell_scope(), url, None).unwrap();
//     },
//     "id:Tauri" => {
//       let url = "https://docs.rs/tauri/1.0.0-rc.3/tauri/index.html ".to_string();
//       shell::open(&event.window().shell_scope(), url, None).unwrap();
//     }
//     _ => {}
//   }
// })