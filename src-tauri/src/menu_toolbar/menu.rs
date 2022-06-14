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
/*  Updated: 2022/06/14 14:55:29 by dnettoRaw                                 */
/*                                                    https://dnetto.dev      */
/* ************************************************************************** */

#[allow(unused_imports)]
use tauri::{Menu, MenuItem, Submenu};


#[allow(unused_imports)]
use crate::menu_toolbar::{menu_android,menu_ios,menu_linux,menu_mac,menu_windows};
use crate::function::social_link;

pub fn dr_menu() -> Menu {

  let mut _my_app : Menu = Menu::new();
  let mut _file   : Menu = Menu::new();
  let mut _edit   : Menu = Menu::new();
  let mut _help   : Menu = Menu::new();

  #[cfg(target_os = "ios")]{
    // unsupported for this tauri vertion
    let _my_app : Menu = menu_ios::get_my_app();
    let _file   : Menu = menu_ios::get_file();
    let _edit   : Menu = menu_ios::get_edit();
    let _help   : Menu = menu_ios::get_help();
  }
  #[cfg(target_os = "android")]{
    // unsupported for this tauri vertion
    let _my_app : Menu = menu_android::get_my_app();
    let _file   : Menu = menu_android::get_file();
    let _edit   : Menu = menu_android::get_edit();
    let _help   : Menu = menu_android::get_help();
  }
  #[cfg(target_os = "linux")]{
    let _my_app : Menu = menu_linux::get_my_app();
    let _file   : Menu = menu_linux::get_file();
    let _edit   : Menu = menu_linux::get_edit();
    let _help   : Menu = menu_linux::get_help();
  }
  #[cfg(target_os = "windows")]{
    let _my_app : Menu = menu_windows::get_my_app();
    let _file   : Menu = menu_windows::get_file();
    let _edit   : Menu = menu_windows::get_edit();
    let _help   : Menu = menu_windows::get_help();
  }
  #[cfg(target_os = "macos")]{
    let _my_app : Menu = menu_mac::get_my_app();
    let _file   : Menu = menu_mac::get_file();
    let _edit   : Menu = menu_mac::get_edit();
    let _help   : Menu = menu_mac::get_help();
  }


  // add all our childs to the menu (order is how they'll appear)

  Menu::new()
    .add_submenu(Submenu::new("app ", _my_app))
    .add_submenu(Submenu::new("File", _file))
    .add_submenu(Submenu::new("Edit", _edit))
    .add_submenu(Submenu::new("Help", _help))

    // another way to do this


}

pub fn dr_event(event: tauri::WindowMenuEvent){  
/*
  the id's is in Hex start at 01 and at ff

  // social section
  01  github
  02  tauri doc
  
  // data base 
  10 init db
  11 update db
  ...

  the id can be anything you want, this is a String, but for me 
  i prefere to keep it in numerical id
  
*/
    let event_name = event.menu_item_id();
    match event_name {
      "01" => {
        social_link::github(event);
      },
      "02" => {
        social_link::my_site(event);
      },
      "03" =>{
        social_link::tauri(event);
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