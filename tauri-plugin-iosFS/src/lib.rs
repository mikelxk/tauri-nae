use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::IosFs;
#[cfg(mobile)]
use mobile::IosFs;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ios-fs APIs.
pub trait IosFsExt<R: Runtime> {
  fn ios_fs(&self) -> &IosFs<R>;
}

impl<R: Runtime, T: Manager<R>> crate::IosFsExt<R> for T {
  fn ios_fs(&self) -> &IosFs<R> {
    self.state::<IosFs<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("ios-fs")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let ios_fs = mobile::init(app, api)?;
      #[cfg(desktop)]
      let ios_fs = desktop::init(app, api)?;
      app.manage(ios_fs);
      Ok(())
    })
    .build()
}

