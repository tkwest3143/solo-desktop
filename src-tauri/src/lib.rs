pub mod data;
pub mod entities;
pub mod repositories;
mod tauri_command;
use tauri_command::{
  user::{create_user, get_all_users, get_user_by_id, update_user},
  work_times::{create_work_time, get_work_time_by_month, update_work_time},
};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::new().build())
    .invoke_handler(tauri::generate_handler![
      get_all_users,
      get_user_by_id,
      create_user,
      update_user,
      get_work_time_by_month,
      create_work_time,
      update_work_time
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
