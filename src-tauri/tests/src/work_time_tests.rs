// Work times backend tests
use sea_orm::{Database, DbConn, DbErr, EntityTrait, Set, Statement, ConnectionTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use chrono::{Local, NaiveDateTime};

// Data structures for testing work times
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

#[derive(Serialize, Deserialize)]
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

// Mock entities for testing work times
pub mod work_times {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "work_times")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub target_day: String,
        pub start: Option<DateTime>,
        pub end: Option<DateTime>,
        pub rest_start: Option<DateTime>,
        pub rest_end: Option<DateTime>,
        pub memo: Option<String>,
        pub user_id: i32,
        pub created_at: DateTime,
        pub updated_at: DateTime,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Create an in-memory SQLite database for testing work times
pub async fn create_test_db() -> Result<DbConn, DbErr> {
    let db_url = "sqlite::memory:";
    
    let db = Database::connect(db_url).await?;
    setup_schema(&db).await?;
    Ok(db)
}

/// Setup database schema for testing work times
async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
    // Create work_times table
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE work_times (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            target_day TEXT NOT NULL,
            start DATETIME,
            end DATETIME,
            rest_start DATETIME,
            rest_end DATETIME,
            memo TEXT,
            user_id INTEGER NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#.to_string(),
    )).await?;
    
    Ok(())
}

// Business logic functions for work time operations
pub async fn get_work_time_by_month_with_db(user_id: i32, target_month: &str, db: &DbConn) -> Result<String, String> {
    let work_times = work_times::Entity::find()
        .filter(work_times::Column::UserId.eq(user_id))
        .filter(work_times::Column::TargetDay.like(format!("{}%", target_month)))
        .all(db)
        .await
        .unwrap();
    
    let mut response_work_times: Vec<ResponseWorkTime> = vec![];
    for work_time in work_times {
        response_work_times.push(ResponseWorkTime {
            id: work_time.id,
            target_day: work_time.target_day,
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

pub async fn create_work_time_with_db(work_time: &str, db: &DbConn) -> Result<String, String> {
    let json_to: WorkTimeForInsert = serde_json::from_str(work_time).unwrap();
    
    let start = if let Some(start_str) = &json_to.start {
        if !start_str.is_empty() {
            Some(NaiveDateTime::parse_from_str(start_str, "%Y-%m-%d %H:%M:%S").unwrap())
        } else {
            None
        }
    } else {
        None
    };
    
    let end = if let Some(end_str) = &json_to.end {
        if !end_str.is_empty() {
            Some(NaiveDateTime::parse_from_str(end_str, "%Y-%m-%d %H:%M:%S").unwrap())
        } else {
            None
        }
    } else {
        None
    };
    
    let rest_start = if let Some(rest_start_str) = &json_to.rest_start {
        if !rest_start_str.is_empty() {
            Some(NaiveDateTime::parse_from_str(rest_start_str, "%Y-%m-%d %H:%M:%S").unwrap())
        } else {
            None
        }
    } else {
        None
    };
    
    let rest_end = if let Some(rest_end_str) = &json_to.rest_end {
        if !rest_end_str.is_empty() {
            Some(NaiveDateTime::parse_from_str(rest_end_str, "%Y-%m-%d %H:%M:%S").unwrap())
        } else {
            None
        }
    } else {
        None
    };
    
    let data = work_times::ActiveModel {
        target_day: Set(json_to.target_day),
        start: Set(start),
        end: Set(end),
        rest_start: Set(rest_start),
        rest_end: Set(rest_end),
        memo: Set(json_to.memo),
        user_id: Set(json_to.user_id),
        created_at: Set(Local::now().naive_local()),
        updated_at: Set(Local::now().naive_local()),
        ..Default::default()
    };
    
    work_times::Entity::insert(data).exec(db).await.unwrap();
    Ok("create_work_time finish".to_string())
}

pub async fn update_work_time_with_db(work_time: &str, db: &DbConn) -> Result<String, String> {
    let json_to: WorkTimeForUpdate = serde_json::from_str(work_time).unwrap();
    
    let work_time_data = work_times::Entity::find()
        .filter(work_times::Column::UserId.eq(json_to.user_id))
        .filter(work_times::Column::TargetDay.eq(&json_to.target_day))
        .one(db)
        .await
        .unwrap();
    
    if work_time_data.is_none() {
        return Err("work time not found".to_string());
    }
    
    let mut data: work_times::ActiveModel = work_time_data.unwrap().into();
    
    if let Some(start_str) = &json_to.start {
        if !start_str.is_empty() {
            data.start = Set(Some(NaiveDateTime::parse_from_str(start_str, "%Y-%m-%d %H:%M:%S").unwrap()));
        }
    }
    
    if let Some(end_str) = &json_to.end {
        if !end_str.is_empty() {
            data.end = Set(Some(NaiveDateTime::parse_from_str(end_str, "%Y-%m-%d %H:%M:%S").unwrap()));
        }
    }
    
    if let Some(rest_start_str) = &json_to.rest_start {
        if !rest_start_str.is_empty() {
            data.rest_start = Set(Some(NaiveDateTime::parse_from_str(rest_start_str, "%Y-%m-%d %H:%M:%S").unwrap()));
        }
    }
    
    if let Some(rest_end_str) = &json_to.rest_end {
        if !rest_end_str.is_empty() {
            data.rest_end = Set(Some(NaiveDateTime::parse_from_str(rest_end_str, "%Y-%m-%d %H:%M:%S").unwrap()));
        }
    }
    
    if json_to.memo.is_some() {
        data.memo = Set(json_to.memo);
    }
    
    data.updated_at = Set(Local::now().naive_local());
    work_times::Entity::update(data).exec(db).await.unwrap();
    Ok("update_work_time finish".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_get_work_time() {
        let db = create_test_db().await.unwrap();
        
        // Create a work time entry
        let work_time_json = r#"{
            "target_day": "2024-01-15",
            "start": "2024-01-15 09:00:00",
            "end": "2024-01-15 17:00:00",
            "rest_start": "2024-01-15 12:00:00",
            "rest_end": "2024-01-15 13:00:00",
            "memo": "Regular work day",
            "user_id": 1
        }"#;
        
        let result = create_work_time_with_db(work_time_json, &db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "create_work_time finish");
        
        // Get work time by month
        let result = get_work_time_by_month_with_db(1, "2024-01", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 1);
        assert_eq!(work_times[0].target_day, "2024-01-15");
        assert_eq!(work_times[0].memo, Some("Regular work day".to_string()));
        assert_eq!(work_times[0].user_id, 1);
    }

    #[tokio::test]
    async fn test_create_work_time_with_optional_fields() {
        let db = create_test_db().await.unwrap();
        
        // Create a minimal work time entry
        let work_time_json = r#"{
            "target_day": "2024-01-16",
            "user_id": 1
        }"#;
        
        let result = create_work_time_with_db(work_time_json, &db).await;
        assert!(result.is_ok());
        
        // Get work time by month
        let result = get_work_time_by_month_with_db(1, "2024-01", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 1);
        assert_eq!(work_times[0].target_day, "2024-01-16");
        assert_eq!(work_times[0].start, None);
        assert_eq!(work_times[0].end, None);
        assert_eq!(work_times[0].memo, None);
    }

    #[tokio::test]
    async fn test_update_work_time() {
        let db = create_test_db().await.unwrap();
        
        // Create a work time entry first
        let work_time_json = r#"{
            "target_day": "2024-01-17",
            "start": "2024-01-17 09:00:00",
            "end": "2024-01-17 17:00:00",
            "user_id": 1
        }"#;
        create_work_time_with_db(work_time_json, &db).await.unwrap();
        
        // Update the work time
        let update_json = r#"{
            "user_id": 1,
            "target_day": "2024-01-17",
            "start": "2024-01-17 08:30:00",
            "end": "2024-01-17 18:00:00",
            "rest_start": "2024-01-17 12:00:00",
            "rest_end": "2024-01-17 13:00:00",
            "memo": "Extended work day"
        }"#;
        
        let result = update_work_time_with_db(update_json, &db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "update_work_time finish");
        
        // Verify the update
        let result = get_work_time_by_month_with_db(1, "2024-01", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 1);
        assert!(work_times[0].start.as_ref().unwrap().contains("08:30:00"));
        assert!(work_times[0].end.as_ref().unwrap().contains("18:00:00"));
        assert_eq!(work_times[0].memo, Some("Extended work day".to_string()));
    }

    #[tokio::test]
    async fn test_update_nonexistent_work_time() {
        let db = create_test_db().await.unwrap();
        
        // Try to update a work time that doesn't exist
        let update_json = r#"{
            "user_id": 1,
            "target_day": "2024-01-20",
            "start": "2024-01-20 09:00:00"
        }"#;
        
        let result = update_work_time_with_db(update_json, &db).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "work time not found");
    }

    #[tokio::test]
    async fn test_get_work_time_by_month_filter() {
        let db = create_test_db().await.unwrap();
        
        // Create work times for different months
        let jan_work_time = r#"{
            "target_day": "2024-01-15",
            "start": "2024-01-15 09:00:00",
            "user_id": 1
        }"#;
        let feb_work_time = r#"{
            "target_day": "2024-02-15",
            "start": "2024-02-15 09:00:00",
            "user_id": 1
        }"#;
        let another_jan_work_time = r#"{
            "target_day": "2024-01-20",
            "start": "2024-01-20 09:00:00",
            "user_id": 1
        }"#;
        
        create_work_time_with_db(jan_work_time, &db).await.unwrap();
        create_work_time_with_db(feb_work_time, &db).await.unwrap();
        create_work_time_with_db(another_jan_work_time, &db).await.unwrap();
        
        // Get January work times
        let result = get_work_time_by_month_with_db(1, "2024-01", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 2);
        assert!(work_times.iter().all(|wt| wt.target_day.starts_with("2024-01")));
        
        // Get February work times
        let result = get_work_time_by_month_with_db(1, "2024-02", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 1);
        assert_eq!(work_times[0].target_day, "2024-02-15");
    }

    #[tokio::test]
    async fn test_get_work_time_by_user_filter() {
        let db = create_test_db().await.unwrap();
        
        // Create work times for different users
        let user1_work_time = r#"{
            "target_day": "2024-01-15",
            "user_id": 1
        }"#;
        let user2_work_time = r#"{
            "target_day": "2024-01-15",
            "user_id": 2
        }"#;
        
        create_work_time_with_db(user1_work_time, &db).await.unwrap();
        create_work_time_with_db(user2_work_time, &db).await.unwrap();
        
        // Get user 1's work times
        let result = get_work_time_by_month_with_db(1, "2024-01", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 1);
        assert_eq!(work_times[0].user_id, 1);
        
        // Get user 2's work times
        let result = get_work_time_by_month_with_db(2, "2024-01", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 1);
        assert_eq!(work_times[0].user_id, 2);
    }

    #[tokio::test]
    async fn test_work_time_json_serialization() {
        let work_time = ResponseWorkTime {
            id: 1,
            target_day: "2024-01-15".to_string(),
            start: Some("2024-01-15 09:00:00".to_string()),
            end: Some("2024-01-15 17:00:00".to_string()),
            rest_start: Some("2024-01-15 12:00:00".to_string()),
            rest_end: Some("2024-01-15 13:00:00".to_string()),
            memo: Some("Test work day".to_string()),
            user_id: 1,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };
        
        let json = serde_json::to_string(&work_time).unwrap();
        assert!(json.contains("2024-01-15"));
        assert!(json.contains("Test work day"));
        
        // Test deserialization of input data
        let input = r#"{
            "target_day": "2024-01-16",
            "start": "2024-01-16 08:00:00",
            "end": "2024-01-16 16:00:00",
            "user_id": 1
        }"#;
        let parsed: WorkTimeForInsert = serde_json::from_str(input).unwrap();
        assert_eq!(parsed.target_day, "2024-01-16");
        assert_eq!(parsed.start, Some("2024-01-16 08:00:00".to_string()));
        assert_eq!(parsed.user_id, 1);
    }

    #[tokio::test]
    async fn test_empty_work_time_month_query() {
        let db = create_test_db().await.unwrap();
        
        // Query for a month with no work times
        let result = get_work_time_by_month_with_db(1, "2025-01", &db).await.unwrap();
        let work_times: Vec<ResponseWorkTime> = serde_json::from_str(&result).unwrap();
        assert_eq!(work_times.len(), 0);
    }
}