use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct UserForInsert {
  pub name: String,
  pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserForUpdate {
  pub id: i32,
  pub name: Option<String>,
  pub email: Option<String>,
  pub last_login_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkTimeForInsert {
  pub target_day: String,
  pub start: Option<String>,
  pub end: Option<String>,
  pub rest_start: Option<String>,
  pub rest_end: Option<String>,
  pub memo: Option<String>,
  pub user_id: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkTimeForUpdate {
  pub user_id: i32,
  pub target_day: String,
  pub start: Option<String>,
  pub end: Option<String>,
  pub rest_start: Option<String>,
  pub rest_end: Option<String>,
  pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetWorkTimeByMonthQuery {
  pub user_id: i32,
  pub target_month: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkTimeSettingForInsert {
  pub title: String,
  pub start: String,
  pub end: String,
  pub rest_start: String,
  pub rest_end: String,
  pub working_unit: i32,
  pub memo: Option<String>,
  pub user_id: i32,
  pub is_default: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkTimeSettingForUpdate {
  pub id: i32,
  pub title: Option<String>,
  pub start: Option<String>,
  pub end: Option<String>,
  pub rest_start: Option<String>,
  pub rest_end: Option<String>,
  pub working_unit: Option<i32>,
  pub memo: Option<String>,
  pub user_id: i32,
  pub is_default: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoCategoryForInsert {
  pub name: String,
  pub memo: Option<String>,
  pub color: Option<String>,
  pub user_id: i32,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct TodoCategoryForUpdate {
  pub id: i32,
  pub name: Option<String>,
  pub memo: Option<String>,
  pub color: Option<String>,
  pub user_id: i32,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItemForInsert {
  pub title: String,
  pub content: Option<String>,
  pub link: Option<String>,
  pub color: Option<String>,
  pub due_date: String,
  pub category_id: Option<i32>,
  pub user_id: i32,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItemForUpdate {
  pub id: i32,
  pub title: Option<String>,
  pub content: Option<String>,
  pub link: Option<String>,
  pub color: Option<String>,
  pub due_date: String,
  pub category_id: Option<i32>,
  pub user_id: i32,
}
