/* ************************************************************************** */
/*                                                                            */
/*  menu_android.rs                                           #####           */
/*                                                         ############       */
/*  By: dnettoRaw <contact@dnetto.dev>                   ###          ###     */
/*                                                      ##    ##  ##    ##    */
/*  obs: tauri don't support yet                              ##  ##          */
/*                                                                            */
/*                                                      ##    ##  ##   ##     */
/*                                                       ###  ######  ###     */
/*  Created: 2022/06/02 16:50:47 by dnettoRaw             #####    ####       */
/*  Updated: 2022/06/10 14:12:01 by dnettoRaw                                 */
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