pub mod user {
  use crate::data::{UserForInsert, UserForUpdate};
  use crate::entities::users;
  use crate::repositories::database::establish_connection;
  use chrono::Local;
  use sea_orm::{EntityTrait, Set};
  use serde::Serialize;
  #[derive(Serialize)]
  pub struct ResponseUser {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub default_work_setting_id: Option<i32>,
    pub last_login_time: Option<String>,
    pub created_at: String,
    pub updated_at: String,
  }

  #[tauri::command]
  pub async fn get_all_users() -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let users = users::Entity::find().all(&db).await.unwrap();
    let mut response_users: Vec<ResponseUser> = vec![];
    for user in users {
      let user = user;
      response_users.push(ResponseUser {
        id: user.id,
        name: user.name,
        email: user.email,
        default_work_setting_id: user.default_work_setting_id,
        last_login_time: user.last_login_time.map(|x| x.to_string()),
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
      });
    }
    Ok(serde_json::to_string(&response_users).unwrap())
  }
  #[tauri::command]
  pub async fn get_user_by_id(id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let user = users::Entity::find_by_id(id).one(&db).await.unwrap();
    if user.is_none() {
      return Err("user not found".to_string());
    }
    let user = user.unwrap();

    Ok(
      serde_json::to_string(&ResponseUser {
        id: user.id,
        name: user.name,
        email: user.email,
        default_work_setting_id: user.default_work_setting_id,
        last_login_time: user.last_login_time.map(|x| x.to_string()),
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
      })
      .unwrap(),
    )
  }
  #[tauri::command]
  pub async fn create_user(user: &str) -> Result<String, String> {
    let json_to_user: UserForInsert = serde_json::from_str(user).unwrap();
    let db = establish_connection().await.unwrap();
    let user = users::ActiveModel {
      name: Set(json_to_user.name.to_owned()),
      email: Set(Some(json_to_user.email)),
      created_at: Set(Local::now().naive_local()),
      updated_at: Set(Local::now().naive_local()),
      ..Default::default()
    };
    users::Entity::insert(user).exec(&db).await.unwrap();
    Ok("create_user finish".to_string())
  }

  #[tauri::command]
  pub async fn update_user(user: &str) -> Result<String, String> {
    let json_to_user: UserForUpdate = serde_json::from_str(user).unwrap();
    let db = establish_connection().await.unwrap();
    let user = users::Entity::find_by_id(json_to_user.id).one(&db).await.unwrap();
    let mut user: users::ActiveModel = user.unwrap().into();
    if json_to_user.name.is_some() {
      user.name = Set(json_to_user.name.unwrap());
    }
    if json_to_user.email.is_some() {
      user.email = Set(Some(json_to_user.email.unwrap()));
    }
    user.updated_at = Set(Local::now().naive_local());
    users::Entity::update(user).exec(&db).await.unwrap();
    Ok("update_user finish".to_string())
  }
}

pub mod work_times {
  use crate::data::WorkTimeForUpdate;
  use crate::repositories::database::establish_connection;
  use crate::{data::WorkTimeForInsert, entities::work_times};
  use chrono::{Local, NaiveDateTime};
  use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

  use serde::Serialize;
  #[derive(Serialize)]
  pub struct ResponseWorkTime {
    pub id: i32,
    pub target_day: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub rest_start: Option<String>,
    pub rest_end: Option<String>,
    pub memo: Option<String>,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
  }
  #[tauri::command]
  pub async fn get_work_time_by_month(user_id: &str, target_month: &str) -> Result<String, String> {
    let db: sea_orm::DatabaseConnection = establish_connection().await.unwrap();
    let work_times: Vec<work_times::Model> = work_times::Entity::find()
      .filter(work_times::Column::UserId.eq(user_id))
      .filter(work_times::Column::TargetDay.contains(target_month))
      .all(&db)
      .await
      .unwrap();
    let mut response_work_times: Vec<ResponseWorkTime> = vec![];
    for work_time in work_times {
      response_work_times.push(ResponseWorkTime {
        id: work_time.id,
        target_day: work_time.target_day.to_string(),
        start: work_time.start.map(|x| x.to_string()),
        end: work_time.end.map(|x| x.to_string()),
        rest_start: work_time.rest_start.map(|x| x.to_string()),
        rest_end: work_time.rest_end.map(|x| x.to_string()),
        memo: work_time.memo,
        user_id: work_time.user_id,
        created_at: work_time.created_at.to_string(),
        updated_at: work_time.updated_at.to_string(),
      });
    }
    Ok(serde_json::to_string(&response_work_times).unwrap())
  }
  #[tauri::command]
  pub async fn create_work_time(work_time: &str) -> Result<String, String> {
    let json_to: WorkTimeForInsert = serde_json::from_str(work_time).unwrap();
    let db = establish_connection().await.unwrap();

    let mut data = work_times::ActiveModel {
      user_id: Set(json_to.user_id),
      target_day: Set(json_to.target_day.to_owned()),
      memo: Set(json_to.memo),
      created_at: Set(Local::now().naive_local()),
      updated_at: Set(Local::now().naive_local()),
      ..Default::default()
    };

    if json_to.start.is_some() {
      data.start = Set(Some(NaiveDateTime::parse_from_str(&json_to.start.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.end.is_some() {
      data.end = Set(Some(NaiveDateTime::parse_from_str(&json_to.end.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.rest_start.is_some() {
      data.rest_start = Set(Some(NaiveDateTime::parse_from_str(&json_to.rest_start.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.rest_end.is_some() {
      data.rest_end = Set(Some(NaiveDateTime::parse_from_str(&json_to.rest_end.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    work_times::Entity::insert(data).exec(&db).await.unwrap();
    Ok("create_work_time finish".to_string())
  }
  #[tauri::command]
  pub async fn update_work_time(work_time: &str) -> Result<String, String> {
    let json_to: WorkTimeForUpdate = serde_json::from_str(work_time).unwrap();
    let db = establish_connection().await.unwrap();

    let data = work_times::Entity::find()
      .filter(work_times::Column::UserId.eq(json_to.user_id))
      .filter(work_times::Column::TargetDay.eq(json_to.target_day.to_owned()))
      .one(&db)
      .await
      .unwrap();
    let mut data: work_times::ActiveModel = data.unwrap().into();
    data.target_day = Set(json_to.target_day);
    if json_to.start.is_some() {
      data.start = Set(Some(NaiveDateTime::parse_from_str(&json_to.start.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.end.is_some() {
      data.end = Set(Some(NaiveDateTime::parse_from_str(&json_to.end.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.rest_start.is_some() {
      data.rest_start = Set(Some(NaiveDateTime::parse_from_str(&json_to.rest_start.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.rest_end.is_some() {
      data.rest_end = Set(Some(NaiveDateTime::parse_from_str(&json_to.rest_end.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.memo.is_some() {
      data.memo = Set(Some(json_to.memo.unwrap()));
    }
    data.updated_at = Set(Local::now().naive_local());
    work_times::Entity::update(data).exec(&db).await.unwrap();
    Ok("update_work_time finish".to_string())
  }
}
