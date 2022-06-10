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
/*  Updated: 2022/06/10 13:02:22 by dnettoRaw                                 */
/*                                                    https://dnetto.dev      */
/* ************************************************************************** */

#[allow(unused_imports)]
use serde_json::Value::Null;
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, WindowBuilder, WindowUrl,AboutMetadata};

use tauri::api::shell;

pub fn dr_menu() -> Menu {
  // #[allow(unused_mut)]
  // let mut disable_item =
  //   CustomMenuItem::new("disable-menu", "Disable menu").accelerator("CmdOrControl+D");
  // #[allow(unused_mut)]
  // let mut test_item = CustomMenuItem::new("test", "Test").accelerator("CmdOrControl+T");
  // #[cfg(target_os = "macos")]
  // {
  //   disable_item = disable_item.native_image(tauri::NativeImage::MenuOnState);
  //   test_item = test_item.native_image(tauri::NativeImage::Add);
  // }

  // // create a submenu
  // let my_sub_menu = Menu::with_items([disable_item.into()]);

  // let my_app_menu = Menu::new()
  //   .add_native_item(MenuItem::Copy)
  //   .add_submenu(Submenu::new("Sub menu", my_sub_menu));

  // let test_menu = Menu::new()
  //   .add_item(CustomMenuItem::new(
  //     "selected/disabled",
  //     "Selected and disabled",
  //   ))
  //   .add_native_item(MenuItem::Separator)
  //   .add_item(test_item);
  let ctx = tauri::generate_context!();
  let mut _my_app = Menu::new().add_native_item(MenuItem::Copy);
  let mut _file = Menu::new();
  let mut _edit = Menu::new();
  let mut _help = Menu::new();


  // add all our childs to the menu (order is how they'll appear)
  Menu::new()
    .add_submenu(Submenu::new(&ctx.package_info().name, _my_app))
    .add_submenu(Submenu::new("File", _file))
    .add_submenu(Submenu::new("Edit", _edit))
    .add_submenu(Submenu::new("Help", _help))
    // .add_submenu(Submenu::new("Help",Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()])))
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