pub mod data;
pub mod entities;
pub mod helper;
pub mod repositories;
mod tauri_command;
use tauri_command::{
  japanese_holiday::{get_all_japanese_holidays, import_japanese_holiday},
  user::{create_user, get_all_users, get_user_by_id, update_user},
  work_time_settings::{create_work_time_setting, get_work_setting_by_user_id, update_work_time_setting},
  work_times::{create_work_time, get_work_time_by_month, update_work_time},
};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::new().level(log::LevelFilter::Info).build())
    .invoke_handler(tauri::generate_handler![
      get_all_users,
      get_user_by_id,
      create_user,
      update_user,
      get_work_time_by_month,
      create_work_time,
      update_work_time,
      get_work_setting_by_user_id,
      create_work_time_setting,
      update_work_time_setting,
      get_all_japanese_holidays,
      import_japanese_holiday
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
