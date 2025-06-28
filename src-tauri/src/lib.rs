pub mod data;
pub mod entities;
pub mod helper;
pub mod repositories;
mod tauri_command;
use tauri_command::{
  japanese_holiday::{get_all_japanese_holidays, import_japanese_holiday},
  todo_categories::{create_todo_category, delete_todo_category, get_todo_categories_by_user_id, update_todo_category},
  todo_items::{
    create_todo_item, delete_all_todo_items, delete_todo_item, get_all_todo_items, get_today_todo_items, get_todo_item_by_id, get_todo_items_by_category_id,
    get_upcoming_todo_items, update_todo_item,
  },
  user::{create_user, delete_user, get_all_users, get_user_by_id, update_user},
  work_time_settings::{create_work_time_setting, get_work_setting_by_user_id, update_work_time_setting},
  work_times::{create_work_time, get_work_time_by_month, update_work_time},
};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_log::Builder::new().level(log::LevelFilter::Info).build())
    .invoke_handler(tauri::generate_handler![
      get_all_users,
      get_user_by_id,
      create_user,
      update_user,
      delete_user,
      get_work_time_by_month,
      create_work_time,
      update_work_time,
      get_work_setting_by_user_id,
      create_work_time_setting,
      update_work_time_setting,
      get_all_japanese_holidays,
      import_japanese_holiday,
      delete_todo_item,
      delete_all_todo_items,
      update_todo_item,
      create_todo_item,
      get_all_todo_items,
      get_today_todo_items,
      get_upcoming_todo_items,
      get_todo_items_by_category_id,
      get_todo_item_by_id,
      get_todo_categories_by_user_id,
      create_todo_category,
      update_todo_category,
      delete_todo_category
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
