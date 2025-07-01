pub mod user {
  use crate::data::{UserForInsert, UserForUpdate};
  use crate::entities::users;
  use crate::repositories::database::establish_connection;
  use chrono::{Local, NaiveDateTime};
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
    if json_to_user.last_login_time.is_some() {
      user.last_login_time = Set(Some(
        NaiveDateTime::parse_from_str(&json_to_user.last_login_time.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap(),
      ));
    }
    user.updated_at = Set(Local::now().naive_local());
    users::Entity::update(user).exec(&db).await.unwrap();
    Ok("update_user finish".to_string())
  }

  #[tauri::command]
  pub async fn delete_user(id: i32) -> Result<String, String> {
    let db = establish_connection().await.map_err(|e| e.to_string())?;
    users::Entity::delete_by_id(id).exec(&db).await.map_err(|e| e.to_string())?;
    Ok("delete_user finish".to_string())
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
    let exist_data = work_times::Entity::find()
      .filter(work_times::Column::UserId.eq(json_to.user_id))
      .filter(work_times::Column::TargetDay.eq(json_to.target_day.to_owned()))
      .one(&db)
      .await
      .unwrap();
    if exist_data.is_some() {
      work_times::Entity::delete_by_id(exist_data.unwrap().id).exec(&db).await.unwrap();
    }

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

pub mod work_time_settings {
  use crate::data::{WorkTimeSettingForInsert, WorkTimeSettingForUpdate};
  use crate::entities::work_settings;
  use crate::repositories::database::establish_connection;
  use chrono::{Local, NaiveDateTime};
  use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

  use serde::Serialize;
  #[derive(Serialize)]
  pub struct ResponseWorkTimeSetting {
    pub id: i32,
    pub title: String,
    pub start: String,
    pub end: String,
    pub rest_start: String,
    pub rest_end: String,
    pub memo: Option<String>,
    pub working_unit: i32,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
  }
  #[tauri::command]
  pub async fn get_work_setting_by_user_id(user_id: i32) -> Result<String, String> {
    let db: sea_orm::DatabaseConnection = establish_connection().await.unwrap();
    let work_time_settings: Vec<work_settings::Model> = work_settings::Entity::find().filter(work_settings::Column::UserId.eq(user_id)).all(&db).await.unwrap();
    let mut response_work_time_settings: Vec<ResponseWorkTimeSetting> = vec![];
    for work_time_setting in work_time_settings {
      response_work_time_settings.push(ResponseWorkTimeSetting {
        id: work_time_setting.id,
        title: work_time_setting.title,
        start: work_time_setting.start.to_string(),
        end: work_time_setting.end.to_string(),
        rest_start: work_time_setting.rest_start.to_string(),
        rest_end: work_time_setting.rest_end.to_string(),
        working_unit: work_time_setting.working_unit,
        memo: work_time_setting.memo,
        user_id: work_time_setting.user_id,
        created_at: work_time_setting.created_at.to_string(),
        updated_at: work_time_setting.updated_at.to_string(),
      });
    }
    Ok(serde_json::to_string(&response_work_time_settings).unwrap())
  }
  #[tauri::command]
  pub async fn create_work_time_setting(body: &str) -> Result<String, String> {
    let json_to: WorkTimeSettingForInsert = serde_json::from_str(body).unwrap();
    let db = establish_connection().await.unwrap();
    let data = work_settings::ActiveModel {
      user_id: Set(json_to.user_id),
      title: Set(json_to.title.to_owned()),
      start: Set(NaiveDateTime::parse_from_str(&json_to.start, "%Y-%m-%d %H:%M:%S").unwrap()),
      end: Set(NaiveDateTime::parse_from_str(&json_to.end, "%Y-%m-%d %H:%M:%S").unwrap()),
      rest_start: Set(NaiveDateTime::parse_from_str(&json_to.rest_start, "%Y-%m-%d %H:%M:%S").unwrap()),
      rest_end: Set(NaiveDateTime::parse_from_str(&json_to.rest_end, "%Y-%m-%d %H:%M:%S").unwrap()),
      working_unit: Set(json_to.working_unit),
      memo: Set(json_to.memo),
      created_at: Set(Local::now().naive_local()),
      updated_at: Set(Local::now().naive_local()),
      ..Default::default()
    };
    log::info!("{:?}", data);
    let result = work_settings::Entity::insert(data).exec(&db).await.unwrap();
    if json_to.is_default {
      let user = crate::entities::users::Entity::find_by_id(json_to.user_id).one(&db).await.unwrap().unwrap();
      let mut user: crate::entities::users::ActiveModel = user.into();
      user.default_work_setting_id = Set(Some(result.last_insert_id));
      crate::entities::users::Entity::update(user).exec(&db).await.unwrap();
    }
    Ok("create_work_settings finish".to_string())
  }
  #[tauri::command]
  pub async fn update_work_time_setting(body: &str) -> Result<String, String> {
    let json_to: WorkTimeSettingForUpdate = serde_json::from_str(body).unwrap();
    let db = establish_connection().await.unwrap();

    let data = work_settings::Entity::find_by_id(json_to.id).one(&db).await.unwrap();
    let mut data: work_settings::ActiveModel = data.unwrap().into();
    if json_to.title.is_some() {
      data.title = Set(json_to.title.unwrap());
    }
    if json_to.start.is_some() {
      data.start = Set(NaiveDateTime::parse_from_str(&json_to.start.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap());
    }
    if json_to.end.is_some() {
      data.end = Set(NaiveDateTime::parse_from_str(&json_to.end.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap());
    }
    if json_to.rest_start.is_some() {
      data.rest_start = Set(NaiveDateTime::parse_from_str(&json_to.rest_start.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap());
    }
    if json_to.rest_end.is_some() {
      data.rest_end = Set(NaiveDateTime::parse_from_str(&json_to.rest_end.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap());
    }
    if json_to.working_unit.is_some() {
      data.working_unit = Set(json_to.working_unit.unwrap());
    }
    if json_to.memo.is_some() {
      data.memo = Set(Some(json_to.memo.unwrap()));
    }
    data.updated_at = Set(Local::now().naive_local());
    work_settings::Entity::update(data).exec(&db).await.unwrap();
    if json_to.is_default {
      let user = crate::entities::users::Entity::find_by_id(json_to.user_id).one(&db).await.unwrap().unwrap();
      let mut user: crate::entities::users::ActiveModel = user.into();
      user.default_work_setting_id = Set(Some(json_to.id));
      crate::entities::users::Entity::update(user).exec(&db).await.unwrap();
    }
    Ok("update_work_time finish".to_string())
  }
}

pub mod japanese_holiday {

  use crate::entities::japanese_holiday;
  use crate::helper::import_csv::read_csv_to_2d_array;
  use crate::repositories::database::establish_connection;
  use chrono::{NaiveDate, NaiveDateTime};
  use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
  use serde::Serialize;
  #[derive(Serialize)]
  pub struct ResponseJapaneseHoliday {
    pub id: i32,
    pub day: String,
    pub subject: String,
  }
  #[tauri::command]
  pub async fn get_all_japanese_holidays(start_year: &str) -> Result<String, String> {
    let start = NaiveDate::parse_from_str(&format!("{}-01-01", start_year), "%Y-%m-%d").unwrap();
    let db = establish_connection().await.unwrap();
    let holidays = japanese_holiday::Entity::find().filter(japanese_holiday::Column::Day.gt(start)).all(&db).await.unwrap();
    let mut response_holidays: Vec<ResponseJapaneseHoliday> = vec![];
    for holiday in holidays {
      response_holidays.push(ResponseJapaneseHoliday {
        id: holiday.id,
        day: holiday.day.to_string(),
        subject: holiday.subject.to_string(),
      });
    }
    Ok(serde_json::to_string(&response_holidays).unwrap())
  }
  #[tauri::command]
  pub async fn import_japanese_holiday() -> Result<String, String> {
    log::info!("Starting Japanese holiday import");
    
    // Add timeout for network request
    let data = tokio::time::timeout(
      std::time::Duration::from_secs(30),
      read_csv_to_2d_array("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv")
    ).await.map_err(|_| "Timeout while downloading holiday data".to_string())?
    .map_err(|e| format!("Failed to download holiday data: {}", e))?;
    
    log::info!("Downloaded holiday data, processing {} rows", data.len());
    
    let mut holidays: Vec<japanese_holiday::ActiveModel> = vec![];
    for result in data {
      let date = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", result[0]), "%Y/%m/%d %H:%M:%S");
      let name = result[1].to_string();
      if date.is_err() {
        log::error!("date parse error : date:{:?} subject: {name}", date);
        continue;
      }
      holidays.push(japanese_holiday::ActiveModel {
        day: Set(date.unwrap()),
        subject: Set(name),
        ..Default::default()
      });
    }
    
    log::info!("Parsed {} holiday entries, connecting to database", holidays.len());
    
    // Add timeout for database connection
    let db = tokio::time::timeout(
      std::time::Duration::from_secs(10),
      establish_connection()
    ).await.map_err(|_| "Timeout while connecting to database".to_string())?
    .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    log::info!("Connected to database, inserting holiday data");
    
    // Add timeout for database operations
    tokio::time::timeout(
      std::time::Duration::from_secs(10),
      japanese_holiday::Entity::delete_many().exec(&db)
    ).await.map_err(|_| "Timeout while deleting old holiday data".to_string())?
    .map_err(|e| format!("Failed to delete old holiday data: {}", e))?;
    
    tokio::time::timeout(
      std::time::Duration::from_secs(10),
      japanese_holiday::Entity::insert_many(holidays).exec(&db)
    ).await.map_err(|_| "Timeout while inserting holiday data".to_string())?
    .map_err(|e| format!("Failed to insert holiday data: {}", e))?;
    
    log::info!("Japanese holiday import completed successfully");
    Ok("create_japanese_holiday finish".to_string())
  }
}

pub mod todo_categories {
  use crate::data::{TodoCategoryForInsert, TodoCategoryForUpdate};
  use crate::entities::todo_categories;
  use crate::repositories::database::establish_connection;
  use chrono::Local;
  use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
  use serde::Serialize;

  #[derive(Serialize)]
  pub struct ResponseTodoCategory {
    pub id: i32,
    pub name: String,
    pub memo: Option<String>,
    pub color: Option<String>,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
  }

  #[tauri::command]
  pub async fn get_todo_categories_by_user_id(user_id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let todo_categories = todo_categories::Entity::find().filter(todo_categories::Column::UserId.eq(user_id)).all(&db).await.unwrap();
    let mut response_categories: Vec<ResponseTodoCategory> = vec![];
    for category in todo_categories {
      response_categories.push(ResponseTodoCategory {
        id: category.id,
        name: category.name,
        memo: category.memo,
        color: category.color,
        user_id: category.user_id,
        created_at: category.created_at.to_string(),
        updated_at: category.updated_at.to_string(),
      });
    }
    Ok(serde_json::to_string(&response_categories).unwrap())
  }

  #[tauri::command]
  pub async fn create_todo_category(category: &str) -> Result<String, String> {
    let json_to: TodoCategoryForInsert = serde_json::from_str(category).unwrap();
    let db = establish_connection().await.unwrap();
    let data = todo_categories::ActiveModel {
      name: Set(json_to.name),
      memo: Set(json_to.memo),
      color: Set(json_to.color),
      user_id: Set(json_to.user_id),
      created_at: Set(Local::now().naive_local()),
      updated_at: Set(Local::now().naive_local()),
      ..Default::default()
    };
    todo_categories::Entity::insert(data).exec(&db).await.unwrap();
    Ok("create_todo_category finish".to_string())
  }

  #[tauri::command]
  pub async fn update_todo_category(category: &str) -> Result<String, String> {
    let json_to: TodoCategoryForUpdate = serde_json::from_str(category).unwrap();
    let db = establish_connection().await.unwrap();
    let data = todo_categories::Entity::find_by_id(json_to.id).one(&db).await.unwrap();
    let mut data: todo_categories::ActiveModel = data.unwrap().into();
    if json_to.name.is_some() {
      data.name = Set(json_to.name.unwrap());
    }
    if json_to.memo.is_some() {
      data.memo = Set(json_to.memo);
    }
    if json_to.color.is_some() {
      data.color = Set(json_to.color);
    }
    data.updated_at = Set(Local::now().naive_local());
    todo_categories::Entity::update(data).exec(&db).await.unwrap();
    Ok("update_todo_category finish".to_string())
  }

  #[tauri::command]
  pub async fn delete_todo_category(id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    todo_categories::Entity::delete_by_id(id).exec(&db).await.unwrap();
    Ok("delete_todo_category finish".to_string())
  }
}

pub mod todo_items {
  use crate::data::{TodoItemForInsert, TodoItemForUpdate};
  use crate::entities::todo_items;
  use crate::repositories::database::establish_connection;
  use chrono::{Local, NaiveDateTime};
  use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

  use serde::Serialize;
  #[derive(Serialize)]
  pub struct ResponseTodoItem {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub link: Option<String>,
    pub color: Option<String>,
    pub priority: Option<String>,
    pub due_date: String,
    pub created_at: String,
    pub updated_at: String,
    pub category_id: Option<i32>,
    pub user_id: i32,
    pub status: Option<String>,
  }

  #[tauri::command]
  pub async fn get_todo_item_by_id(id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let todo_item = todo_items::Entity::find_by_id(id).one(&db).await.unwrap();
    if todo_item.is_none() {
      return Err("Todo item not found".to_string());
    }
    let todo_item = todo_item.unwrap();
    let response_todo_item = ResponseTodoItem {
      id: todo_item.id,
      title: todo_item.title.unwrap_or_default(),
      content: todo_item.content,
      link: todo_item.link,
      color: todo_item.color,
      priority: todo_item.priority,
      due_date: todo_item.due_date.to_string(),
      created_at: todo_item.created_at.to_string(),
      updated_at: todo_item.updated_at.to_string(),
      category_id: todo_item.category_id,
      user_id: todo_item.user_id,
      status: todo_item.status,
    };
    Ok(serde_json::to_string(&response_todo_item).unwrap())
  }

  #[tauri::command]
  pub async fn get_all_todo_items(_user_id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let todo_items = todo_items::Entity::find().all(&db).await.unwrap();
    let mut response_todo_items: Vec<ResponseTodoItem> = vec![];
    for todo_item in todo_items {
      response_todo_items.push(ResponseTodoItem {
        id: todo_item.id,
        title: todo_item.title.unwrap_or_default(),
        content: todo_item.content,
        link: todo_item.link,
        color: todo_item.color,
        priority: todo_item.priority,
        due_date: todo_item.due_date.to_string(),
        created_at: todo_item.created_at.to_string(),
        updated_at: todo_item.updated_at.to_string(),
        category_id: todo_item.category_id,
        user_id: todo_item.user_id,
        status: todo_item.status,
      });
    }
    Ok(serde_json::to_string(&response_todo_items).unwrap())
  }

  #[tauri::command]
  pub async fn get_today_todo_items(_user_id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let today = Local::now().date_naive();
    let start_of_day = today.and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = today.and_hms_opt(23, 59, 59).unwrap();

    let todo_items = todo_items::Entity::find().filter(todo_items::Column::DueDate.between(start_of_day, end_of_day)).all(&db).await.unwrap();
    let mut response_todo_items: Vec<ResponseTodoItem> = vec![];
    for todo_item in todo_items {
      response_todo_items.push(ResponseTodoItem {
        id: todo_item.id,
        title: todo_item.title.unwrap_or_default(),
        content: todo_item.content,
        link: todo_item.link,
        color: todo_item.color,
        priority: todo_item.priority,
        due_date: todo_item.due_date.to_string(),
        created_at: todo_item.created_at.to_string(),
        updated_at: todo_item.updated_at.to_string(),
        category_id: todo_item.category_id,
        user_id: todo_item.user_id,
        status: todo_item.status,
      });
    }
    Ok(serde_json::to_string(&response_todo_items).unwrap())
  }

  #[tauri::command]
  pub async fn get_upcoming_todo_items(_user_id: i32, days: Option<i32>) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let today = Local::now().date_naive();
    let days_ahead = days.unwrap_or(7); // Default to 7 days if not specified
    let end_date = today + chrono::Duration::days(days_ahead as i64);
    let start_of_today = today.and_hms_opt(0, 0, 0).unwrap();
    let end_of_period = end_date.and_hms_opt(23, 59, 59).unwrap();

    let todo_items = todo_items::Entity::find().filter(todo_items::Column::DueDate.between(start_of_today, end_of_period)).all(&db).await.unwrap();
    let mut response_todo_items: Vec<ResponseTodoItem> = vec![];
    for todo_item in todo_items {
      response_todo_items.push(ResponseTodoItem {
        id: todo_item.id,
        title: todo_item.title.unwrap_or_default(),
        content: todo_item.content,
        link: todo_item.link,
        color: todo_item.color,
        priority: todo_item.priority,
        due_date: todo_item.due_date.to_string(),
        created_at: todo_item.created_at.to_string(),
        updated_at: todo_item.updated_at.to_string(),
        category_id: todo_item.category_id,
        user_id: todo_item.user_id,
        status: todo_item.status,
      });
    }
    Ok(serde_json::to_string(&response_todo_items).unwrap())
  }

  #[tauri::command]
  pub async fn get_todo_items_by_category_id(category_id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    let todo_items = todo_items::Entity::find().filter(todo_items::Column::CategoryId.eq(category_id)).all(&db).await.unwrap();
    let mut response_todo_items: Vec<ResponseTodoItem> = vec![];
    for todo_item in todo_items {
      response_todo_items.push(ResponseTodoItem {
        id: todo_item.id,
        title: todo_item.title.unwrap_or_default(),
        content: todo_item.content,
        link: todo_item.link,
        color: todo_item.color,
        priority: todo_item.priority,
        due_date: todo_item.due_date.to_string(),
        created_at: todo_item.created_at.to_string(),
        updated_at: todo_item.updated_at.to_string(),
        category_id: todo_item.category_id,
        user_id: todo_item.user_id,
        status: todo_item.status,
      });
    }
    Ok(serde_json::to_string(&response_todo_items).unwrap())
  }
  #[tauri::command]
  pub async fn create_todo_item(todo_item: &str) -> Result<String, String> {
    let json_to: TodoItemForInsert = serde_json::from_str(todo_item).unwrap();
    let db = establish_connection().await.unwrap();
    let data = todo_items::ActiveModel {
      title: Set(Some(json_to.title)),
      content: Set(json_to.content),
      link: Set(json_to.link),
      color: Set(json_to.color),
      priority: Set(json_to.priority),
      due_date: Set(NaiveDateTime::parse_from_str(&json_to.due_date, "%Y-%m-%d %H:%M:%S").unwrap()),
      category_id: Set(json_to.category_id),
      user_id: Set(json_to.user_id),
      status: Set(json_to.status.or_else(|| Some("incomplete".to_string()))),
      created_at: Set(Local::now().naive_local()),
      updated_at: Set(Local::now().naive_local()),
      ..Default::default()
    };
    todo_items::Entity::insert(data).exec(&db).await.unwrap();
    Ok("create_todo_item finish".to_string())
  }
  #[tauri::command]
  pub async fn update_todo_item(todo_item: &str) -> Result<String, String> {
    let json_to: TodoItemForUpdate = serde_json::from_str(todo_item).unwrap();
    let db = establish_connection().await.unwrap();
    let data = todo_items::Entity::find_by_id(json_to.id).one(&db).await.unwrap();
    let mut data: todo_items::ActiveModel = data.unwrap().into();
    if json_to.title.is_some() {
      data.title = Set(json_to.title);
    }
    if json_to.content.is_some() {
      data.content = Set(json_to.content);
    }
    if json_to.link.is_some() {
      data.link = Set(json_to.link);
    }
    if json_to.color.is_some() {
      data.color = Set(json_to.color);
    }
    if json_to.priority.is_some() {
      data.priority = Set(json_to.priority);
    }
    if !json_to.due_date.is_empty() {
      data.due_date = Set(NaiveDateTime::parse_from_str(&json_to.due_date, "%Y-%m-%d %H:%M:%S").unwrap());
    }
    if json_to.category_id.is_some() {
      data.category_id = Set(json_to.category_id);
    }
    data.user_id = Set(json_to.user_id);
    if json_to.status.is_some() {
      data.status = Set(json_to.status);
    }
    data.updated_at = Set(Local::now().naive_local());
    todo_items::Entity::update(data).exec(&db).await.unwrap();
    Ok("update_todo_item finish".to_string())
  }
  #[tauri::command]
  pub async fn delete_todo_item(id: i32) -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    todo_items::Entity::delete_by_id(id).exec(&db).await.unwrap();
    Ok("delete_todo_item finish".to_string())
  }
  #[tauri::command]
  pub async fn delete_all_todo_items() -> Result<String, String> {
    let db = establish_connection().await.unwrap();
    todo_items::Entity::delete_many().exec(&db).await.unwrap();
    Ok("delete_all_todo_items finish".to_string())
  }
}
