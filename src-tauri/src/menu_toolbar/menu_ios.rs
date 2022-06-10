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
/*  Updated: 2022/06/10 20:06:42 by dnettoRaw                                 */
/*                                                    https://dnetto.dev      */
/* ************************************************************************** */

#![allow(dead_code)]

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