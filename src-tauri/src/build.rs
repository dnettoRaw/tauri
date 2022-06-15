/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: build.rs                             */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:11:41 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/15 16:51:46 by:dnettoRaw     */
/*    ###########                                             */

use tauri_build::{try_build, Attributes, WindowsAttributes};

fn main() {
  if let Err(error) = try_build(
    Attributes::new()
      .windows_attributes(WindowsAttributes::new().window_icon_path("../icons/icon.ico")),
  ) {
    panic!("error found during tauri-build: {:#?}", error);
  }
}