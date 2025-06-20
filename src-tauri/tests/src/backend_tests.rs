// Standalone business logic tests for the backend
use sea_orm::{Database, DbConn, DbErr, EntityTrait, Set, Statement, ConnectionTrait};
use serde::{Deserialize, Serialize};
use chrono::{Local, NaiveDateTime};
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

// Data structures for testing
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

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub default_work_setting_id: Option<i32>,
    pub last_login_time: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Mock entities for testing
pub mod users {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub email: Option<String>,
        pub default_work_setting_id: Option<i32>,
        pub last_login_time: Option<DateTime>,
        pub created_at: DateTime,
        pub updated_at: DateTime,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Create an in-memory SQLite database for testing
pub async fn create_test_db() -> Result<DbConn, DbErr> {
    let db_url = "sqlite::memory:";
    
    let db = Database::connect(db_url).await?;
    setup_schema(&db).await?;
    Ok(db)
}

/// Setup database schema for testing
async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
    // Create users table
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT,
            default_work_setting_id INTEGER,
            last_login_time DATETIME,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#.to_string(),
    )).await?;
    
    Ok(())
}

// Business logic functions for user operations
pub async fn get_all_users_with_db(db: &DbConn) -> Result<String, String> {
    let users = users::Entity::find().all(db).await.unwrap();
    let mut response_users: Vec<ResponseUser> = vec![];
    for user in users {
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

pub async fn get_user_by_id_with_db(id: i32, db: &DbConn) -> Result<String, String> {
    let user = users::Entity::find_by_id(id).one(db).await.unwrap();
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

pub async fn create_user_with_db(user: &str, db: &DbConn) -> Result<String, String> {
    let json_to_user: UserForInsert = serde_json::from_str(user).unwrap();
    let user = users::ActiveModel {
        name: Set(json_to_user.name.to_owned()),
        email: Set(Some(json_to_user.email)),
        created_at: Set(Local::now().naive_local()),
        updated_at: Set(Local::now().naive_local()),
        ..Default::default()
    };
    users::Entity::insert(user).exec(db).await.unwrap();
    Ok("create_user finish".to_string())
}

pub async fn update_user_with_db(user: &str, db: &DbConn) -> Result<String, String> {
    let json_to_user: UserForUpdate = serde_json::from_str(user).unwrap();
    let user = users::Entity::find_by_id(json_to_user.id).one(db).await.unwrap();
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
    users::Entity::update(user).exec(db).await.unwrap();
    Ok("update_user finish".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_users_empty() {
        let db = create_test_db().await.unwrap();
        let result = get_all_users_with_db(&db).await.unwrap();
        let users: Vec<ResponseUser> = serde_json::from_str(&result).unwrap();
        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let db = create_test_db().await.unwrap();
        
        // Create a user
        let user_json = r#"{"name": "Test User", "email": "test@example.com"}"#;
        let result = create_user_with_db(user_json, &db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "create_user finish");
        
        // Get all users
        let result = get_all_users_with_db(&db).await.unwrap();
        let users: Vec<ResponseUser> = serde_json::from_str(&result).unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "Test User");
        assert_eq!(users[0].email, Some("test@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let db = create_test_db().await.unwrap();
        
        // Create a user first
        let user_json = r#"{"name": "Test User", "email": "test@example.com"}"#;
        create_user_with_db(user_json, &db).await.unwrap();
        
        // Get user by ID
        let result = get_user_by_id_with_db(1, &db).await;
        assert!(result.is_ok());
        
        let user: ResponseUser = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, Some("test@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        let db = create_test_db().await.unwrap();
        
        let result = get_user_by_id_with_db(999, &db).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "user not found");
    }

    #[tokio::test]
    async fn test_update_user() {
        let db = create_test_db().await.unwrap();
        
        // Create a user first
        let user_json = r#"{"name": "Test User", "email": "test@example.com"}"#;
        create_user_with_db(user_json, &db).await.unwrap();
        
        // Update the user
        let update_json = r#"{"id": 1, "name": "Updated User", "email": "updated@example.com"}"#;
        let result = update_user_with_db(update_json, &db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "update_user finish");
        
        // Verify the update
        let result = get_user_by_id_with_db(1, &db).await.unwrap();
        let user: ResponseUser = serde_json::from_str(&result).unwrap();
        assert_eq!(user.name, "Updated User");
        assert_eq!(user.email, Some("updated@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_update_user_partial() {
        let db = create_test_db().await.unwrap();
        
        // Create a user first
        let user_json = r#"{"name": "Test User", "email": "test@example.com"}"#;
        create_user_with_db(user_json, &db).await.unwrap();
        
        // Update only the name
        let update_json = r#"{"id": 1, "name": "Updated Name"}"#;
        let result = update_user_with_db(update_json, &db).await;
        assert!(result.is_ok());
        
        // Verify the update
        let result = get_user_by_id_with_db(1, &db).await.unwrap();
        let user: ResponseUser = serde_json::from_str(&result).unwrap();
        assert_eq!(user.name, "Updated Name");
        assert_eq!(user.email, Some("test@example.com".to_string())); // Should remain unchanged
    }

    #[tokio::test]
    async fn test_create_user_with_direct_db_insert() {
        let db = create_test_db().await.unwrap();
        
        // Direct database insert to test the entity
        let user = users::ActiveModel {
            name: Set("Direct User".to_string()),
            email: Set(Some("direct@example.com".to_string())),
            created_at: Set(Local::now().naive_local()),
            updated_at: Set(Local::now().naive_local()),
            ..Default::default()
        };
        
        let result = users::Entity::insert(user).exec(&db).await;
        assert!(result.is_ok());
        
        // Verify the user was created
        let result = get_all_users_with_db(&db).await.unwrap();
        let users: Vec<ResponseUser> = serde_json::from_str(&result).unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "Direct User");
    }

    #[tokio::test]
    async fn test_json_serialization() {
        let user = ResponseUser {
            id: 1,
            name: "Test User".to_string(),
            email: Some("test@example.com".to_string()),
            default_work_setting_id: None,
            last_login_time: None,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };
        
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("Test User"));
        assert!(json.contains("test@example.com"));
        
        // Test deserialization of input data
        let input = r#"{"name": "John Doe", "email": "john@example.com"}"#;
        let parsed: UserForInsert = serde_json::from_str(input).unwrap();
        assert_eq!(parsed.name, "John Doe");
        assert_eq!(parsed.email, "john@example.com");
    }

    #[tokio::test]
    async fn test_database_schema_setup() {
        let db = create_test_db().await.unwrap();
        
        // Test that we can insert data directly into the database
        db.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "INSERT INTO users (name, email, created_at, updated_at) VALUES ('Schema Test', 'schema@test.com', '2024-01-01 00:00:00', '2024-01-01 00:00:00')".to_string(),
        )).await.unwrap();
        
        // Verify the data was inserted
        let result = get_all_users_with_db(&db).await.unwrap();
        let users: Vec<ResponseUser> = serde_json::from_str(&result).unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "Schema Test");
    }
}