/* ************************************************************************** */
/*                                                                            */
/*  menu_ios.rs                                               #####           */
/*                                                         ############       */
/*  By: dnettoRaw <contact@dnetto.dev>                   ###          ###     */
/*                                                      ##    ##  ##    ##    */
/*  obs:                                                      ##  ##          */
/*                                                                            */
/*                                                      ##    ##  ##   ##     */
/*                                                       ###  ######  ###     */
/*  Created: 2022/06/10 13:56:44 by dnettoRaw             #####    ####       */
/*  Updated: 2022/06/10 14:11:46 by dnettoRaw                                 */
/*                                                    https://dnetto.dev      */
/* ************************************************************************** */


use tauri::{Menu};

pub fn get_my_app() -> Menu {
    Menu::with_items([])
  }
  pub fn get_file() -> Menu {
    Menu::with_items([])
  }
  pub fn get_edit() -> Menu {
    Menu::with_items([])
  }
  pub fn get_help() -> Menu {
    Menu::with_items([])
  }