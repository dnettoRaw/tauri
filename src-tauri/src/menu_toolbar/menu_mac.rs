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
/*  Updated: 2022/06/14 19:56:45 by dnettoRaw                                 */
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
    
pub fn get_my_app() -> Menu {
  // for somme raison i dont know this dont work anymore 
  // let  medata = AboutMetadata::default().authors(vec![String::from("dnetto") , String::from("Raw"), String::from("sommeone")])
      // .comments(String::from("teste 1"))
      // .copyright(String::from("teste 2"))
      // .license(String::from("teste 3"))
      // .version(String::from("teste 4"))
      // .website(String::from("dnetto.dev"));

  Menu::with_items([
        MenuItem::About(String::from("myAppInDev"),AboutMetadata::default()).into(),
        MenuItem::Separator.into(),
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
    //  firts is Id this need to be same in your functions, secound is the name displayed
    CustomMenuItem::new("01", "dnetto github").into(),
    CustomMenuItem::new("02", "dnetto site").into(),
    CustomMenuItem::new("03", "tauri github").into(),
  ])
}