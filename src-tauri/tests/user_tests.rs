// Test module for user commands
use solo_lib::data::{UserForInsert, UserForUpdate};
use solo_lib::entities::users;
use solo_lib::test_utils::test_utils::create_test_db;
use solo_lib::tauri_command::user::{
    get_all_users_with_db, get_user_by_id_with_db, 
    create_user_with_db, update_user_with_db, ResponseUser
};
use sea_orm::{EntityTrait, Set};
use chrono::Local;

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