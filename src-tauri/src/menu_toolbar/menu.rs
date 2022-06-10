/* ************************************************************************** */
/*                                                                            */
/*  menu.rs                                                   #####           */
/*                                                         ############       */
/*  By: dnettoRaw <contact@dnetto.dev>                   ###          ###     */
/*                                                      ##    ##  ##    ##    */
/*  obs:                                                      ##  ##          */
/*                                                                            */
/*                                                      ##    ##  ##   ##     */
/*                                                       ###  ######  ###     */
/*  Created: 2022/06/03 15:36:40 by dnettoRaw             #####    ####       */
/*  Updated: 2022/06/10 20:03:46 by dnettoRaw                                 */
/*                                                    https://dnetto.dev      */
/* ************************************************************************** */


use tauri::{Manager, Menu, MenuItem, Submenu};

use tauri::api::shell;

#[allow(unused_imports)]
use crate::menu_toolbar::{menu_android,menu_ios,menu_linux,menu_mac,menu_windows};

pub fn dr_menu() -> Menu {

  let mut _my_app = Menu::new().add_native_item(MenuItem::Copy);
  let mut _file = Menu::new();
  let mut _edit = Menu::new();
  let mut _help = Menu::new();

  #[cfg(target_os = "ios")]{
    // unsupported for this tauri vertion
    _my_app = menu_ios::get_my_app();
    _file   = menu_ios::get_file();
    _edit   = menu_ios::get_edit();
    _help   = menu_ios::get_help();
  }
  #[cfg(target_os = "android")]{
    // unsupported for this tauri vertion
    _my_app = menu_android::get_my_app();
    _file   = menu_android::get_file();
    _edit   = menu_android::get_edit();
    _help   = menu_android::get_help();
  }
  #[cfg(target_os = "linux")]{
    _my_app = menu_linux::get_my_app();
    _file   = menu_linux::get_file();
    _edit   = menu_linux::get_edit();
    _help   = menu_linux::get_help();
  }
  #[cfg(target_os = "windows")]{
    _my_app = menu_windows::get_my_app();
    _file   = menu_windows::get_file();
    _edit   = menu_windows::get_edit();
    _help   = menu_windows::get_help();
  }
  #[cfg(target_os = "macos")]{
    _my_app = menu_mac::get_my_app();
    _file   = menu_mac::get_file();
    _edit   = menu_mac::get_edit();
    _help   = menu_mac::get_help();
  }


  // add all our childs to the menu (order is how they'll appear)
  Menu::new()
    .add_submenu(Submenu::new("app ", _my_app))
    .add_submenu(Submenu::new("File", _file))
    .add_submenu(Submenu::new("Edit", _edit))
    .add_submenu(Submenu::new("Help", _help))
}

pub fn dr_event(event: tauri::WindowMenuEvent){  
    let event_name = event.menu_item_id();
    match event_name {
      "Learn More" => {
        let url = "https://github.com/probablykasper/tauri-template".to_string();
        shell::open(&event.window().shell_scope(), url, None).unwrap();
      }
      _ => {}
    }
}


// Menu::with_items([
//     #[cfg(target_os = "macos")]
//     MenuEntry::Submenu(Submenu::new(
//       &ctx.package_info().name,
//       Menu::with_items([
//         MenuItem::About(ctx.package_info().name.clone(),aboutMe).into(),
//         MenuItem::Separator.into(),
//         MenuItem::Services.into(),
//         MenuItem::Separator.into(),
//         MenuItem::Hide.into(),
//         MenuItem::HideOthers.into(),
//         MenuItem::ShowAll.into(),
//         MenuItem::Separator.into(),
//         MenuItem::Quit.into(),
//       ]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "File",
//       Menu::with_items([MenuItem::CloseWindow.into()]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "Edit",
//       Menu::with_items([
//         MenuItem::Undo.into(),
//         MenuItem::Redo.into(),
//         MenuItem::Separator.into(),
//         MenuItem::Cut.into(),
//         MenuItem::Copy.into(),
//         MenuItem::Paste.into(),
//         #[cfg(not(target_os = "macos"))]
//         MenuItem::Separator.into(),
//         MenuItem::SelectAll.into(),
//       ]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "View",
//       Menu::with_items([MenuItem::EnterFullScreen.into()]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "Window",
//       Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
//     )),
//     // You should always have a Help menu on macOS because it will automatically
//     // show a menu search field
//     MenuEntry::Submenu(Submenu::new(
//       "Help",
//       Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
//     )),
//   ])