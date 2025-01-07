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
