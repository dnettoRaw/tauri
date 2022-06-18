/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: build.rs                             */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:11:41 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/18 18:04:15 by:dnettoRaw     */
/*    ###########                                             */

#[allow(unused_imports)]
use tauri_build::{try_build, Attributes, WindowsAttributes};

fn main() {
  if let Err(error) = try_build(
    // #[cfg(any(target_os = "linux", target_os = "macos"))]
    Attributes::new().windows_attributes(WindowsAttributes::new().window_icon_path("../icons/icon.ico")),
    // #[cfg(target_os = "windows")]
    // Attributes::new().windows_attributes(WindowsAttributes::new().window_icon_path("..\\icons\\icon.ico")),
  ) {
    panic!("error found during tauri-build: {:#?}", error);
  }
}