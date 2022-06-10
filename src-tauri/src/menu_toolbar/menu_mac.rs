/* ************************************************************************** */
/*                                                                            */
/*  menu_mac.rs                                               #####           */
/*                                                         ############       */
/*  By: dnettoRaw <contact@dnetto.dev>                   ###          ###     */
/*                                                      ##    ##  ##    ##    */
/*  obs:                                                      ##  ##          */
/*                                                                            */
/*                                                      ##    ##  ##   ##     */
/*                                                       ###  ######  ###     */
/*  Created: 2022/06/02 16:54:27 by dnettoRaw             #####    ####       */
/*  Updated: 2022/06/10 20:07:10 by dnettoRaw                                 */
/*                                                    https://dnetto.dev      */
/* ************************************************************************** */

#![allow(dead_code)]
#[allow(unused_imports)]
use tauri::{CustomMenuItem,
    Manager,
    Menu,
    MenuEntry,
    MenuItem,
    Submenu,
    WindowBuilder,
    WindowUrl,
    AboutMetadata};
    
// pub fn dn_getMacMenus() -> Menu{
//     Menu::with_items([
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
// }

pub fn get_my_app() -> Menu {
  Menu::with_items([
        // MenuItem::About(ctx.package_info().name.clone(),aboutMe).into(),
        // MenuItem::Separator.into(),
        MenuItem::Services.into(),
        MenuItem::Separator.into(),
        MenuItem::Hide.into(),
        MenuItem::HideOthers.into(),
        MenuItem::ShowAll.into(),
        MenuItem::Separator.into(),
        MenuItem::Quit.into(),
  ])
}
pub fn get_file() -> Menu {
  Menu::with_items([
    MenuItem::CloseWindow.into(),
  ])
}
pub fn get_edit() -> Menu {
  Menu::with_items([
    MenuItem::Undo.into(),
    MenuItem::Redo.into(),
    MenuItem::Separator.into(),
    MenuItem::Cut.into(),
    MenuItem::Copy.into(),
    MenuItem::Paste.into(),
    MenuItem::Separator.into(),
    MenuItem::SelectAll.into(),
  ])
}
pub fn get_help() -> Menu {
  Menu::with_items([
    CustomMenuItem::new("Learn More", "Learn More").into(),
  ])
}