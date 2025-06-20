// Todo items backend tests
use sea_orm::{Database, DbConn, DbErr, EntityTrait, Set, Statement, ConnectionTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use chrono::{Local, NaiveDateTime};

// Data structures for testing todo items
#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItemForInsert {
    pub title: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub color: Option<String>,
    pub priority: Option<i32>,
    pub due_date: String,
    pub category_id: Option<i32>,
    pub status: Option<i32>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItemForUpdate {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub color: Option<String>,
    pub priority: Option<i32>,
    pub due_date: String,
    pub category_id: Option<i32>,
    pub status: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTodoItem {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub color: Option<String>,
    pub priority: Option<i32>,
    pub due_date: String,
    pub category_id: Option<i32>,
    pub status: Option<i32>,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

// Mock entities for testing todo items
pub mod todo_items {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "todo_items")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub title: Option<String>,
        pub content: Option<String>,
        pub link: Option<String>,
        pub color: Option<String>,
        pub priority: Option<i32>,
        pub due_date: Option<DateTime>,
        pub category_id: Option<i32>,
        pub status: Option<i32>,
        pub user_id: i32,
        pub created_at: DateTime,
        pub updated_at: DateTime,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Create an in-memory SQLite database for testing todo items
pub async fn create_test_db() -> Result<DbConn, DbErr> {
    let db_url = "sqlite::memory:";
    
    let db = Database::connect(db_url).await?;
    setup_schema(&db).await?;
    Ok(db)
}

/// Setup database schema for testing todo items
async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
    // Create todo_items table
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE todo_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            content TEXT,
            link TEXT,
            color TEXT,
            priority INTEGER,
            due_date DATETIME,
            category_id INTEGER,
            status INTEGER,
            user_id INTEGER NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#.to_string(),
    )).await?;
    
    Ok(())
}

// Business logic functions for todo item operations
pub async fn get_all_todo_items_with_db(db: &DbConn) -> Result<String, String> {
    let todo_items = todo_items::Entity::find().all(db).await.unwrap();
    let mut response_todo_items: Vec<ResponseTodoItem> = vec![];
    for item in todo_items {
        response_todo_items.push(ResponseTodoItem {
            id: item.id,
            title: item.title,
            content: item.content,
            link: item.link,
            color: item.color,
            priority: item.priority,
            due_date: item.due_date.map(|x| x.to_string()).unwrap_or_default(),
            category_id: item.category_id,
            status: item.status,
            user_id: item.user_id,
            created_at: item.created_at.to_string(),
            updated_at: item.updated_at.to_string(),
        });
    }
    Ok(serde_json::to_string(&response_todo_items).unwrap())
}

pub async fn get_todo_items_by_category_id_with_db(category_id: i32, db: &DbConn) -> Result<String, String> {
    let todo_items = todo_items::Entity::find()
        .filter(todo_items::Column::CategoryId.eq(category_id))
        .all(db)
        .await
        .unwrap();
    let mut response_todo_items: Vec<ResponseTodoItem> = vec![];
    for item in todo_items {
        response_todo_items.push(ResponseTodoItem {
            id: item.id,
            title: item.title,
            content: item.content,
            link: item.link,
            color: item.color,
            priority: item.priority,
            due_date: item.due_date.map(|x| x.to_string()).unwrap_or_default(),
            category_id: item.category_id,
            status: item.status,
            user_id: item.user_id,
            created_at: item.created_at.to_string(),
            updated_at: item.updated_at.to_string(),
        });
    }
    Ok(serde_json::to_string(&response_todo_items).unwrap())
}

pub async fn create_todo_item_with_db(todo_item: &str, db: &DbConn) -> Result<String, String> {
    let json_to: TodoItemForInsert = serde_json::from_str(todo_item).unwrap();
    let due_date = if json_to.due_date.is_empty() {
        None
    } else {
        Some(NaiveDateTime::parse_from_str(&json_to.due_date, "%Y-%m-%d %H:%M:%S").unwrap())
    };
    
    let data = todo_items::ActiveModel {
        title: Set(json_to.title),
        content: Set(json_to.content),
        link: Set(json_to.link),
        color: Set(json_to.color),
        priority: Set(json_to.priority),
        due_date: Set(due_date),
        category_id: Set(json_to.category_id),
        status: Set(json_to.status),
        user_id: Set(json_to.user_id),
        created_at: Set(Local::now().naive_local()),
        updated_at: Set(Local::now().naive_local()),
        ..Default::default()
    };
    todo_items::Entity::insert(data).exec(db).await.unwrap();
    Ok("create_todo_item finish".to_string())
}

pub async fn update_todo_item_with_db(todo_item: &str, db: &DbConn) -> Result<String, String> {
    let json_to: TodoItemForUpdate = serde_json::from_str(todo_item).unwrap();
    let data = todo_items::Entity::find_by_id(json_to.id).one(db).await.unwrap();
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
        data.due_date = Set(Some(NaiveDateTime::parse_from_str(&json_to.due_date, "%Y-%m-%d %H:%M:%S").unwrap()));
    }
    if json_to.category_id.is_some() {
        data.category_id = Set(json_to.category_id);
    }
    if json_to.status.is_some() {
        data.status = Set(json_to.status);
    }
    data.updated_at = Set(Local::now().naive_local());
    todo_items::Entity::update(data).exec(db).await.unwrap();
    Ok("update_todo_item finish".to_string())
}

pub async fn delete_todo_item_with_db(id: i32, db: &DbConn) -> Result<String, String> {
    todo_items::Entity::delete_by_id(id).exec(db).await.unwrap();
    Ok("delete_todo_item finish".to_string())
}

pub async fn delete_all_todo_items_with_db(db: &DbConn) -> Result<String, String> {
    todo_items::Entity::delete_many().exec(db).await.unwrap();
    Ok("delete_all_todo_items finish".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_todo_items_empty() {
        let db = create_test_db().await.unwrap();
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 0);
    }

    #[tokio::test]
    async fn test_create_and_get_todo_item() {
        let db = create_test_db().await.unwrap();
        
        // Create a todo item
        let item_json = r#"{
            "title": "Test Task",
            "content": "Test Content",
            "link": "https://example.com",
            "color": "red",
            "priority": 1,
            "due_date": "2024-12-31 23:59:59",
            "category_id": 1,
            "status": 0,
            "user_id": 1
        }"#;
        let result = create_todo_item_with_db(item_json, &db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "create_todo_item finish");
        
        // Get all todo items
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, Some("Test Task".to_string()));
        assert_eq!(items[0].content, Some("Test Content".to_string()));
        assert_eq!(items[0].priority, Some(1));
        assert_eq!(items[0].user_id, 1);
    }

    #[tokio::test]
    async fn test_create_todo_item_without_optional_fields() {
        let db = create_test_db().await.unwrap();
        
        // Create a minimal todo item
        let item_json = r#"{
            "due_date": "",
            "user_id": 1
        }"#;
        let result = create_todo_item_with_db(item_json, &db).await;
        assert!(result.is_ok());
        
        // Get all todo items
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, None);
        assert_eq!(items[0].content, None);
        assert_eq!(items[0].user_id, 1);
    }

    #[tokio::test]
    async fn test_update_todo_item() {
        let db = create_test_db().await.unwrap();
        
        // Create a todo item first
        let item_json = r#"{
            "title": "Original Task",
            "content": "Original Content",
            "due_date": "",
            "user_id": 1
        }"#;
        create_todo_item_with_db(item_json, &db).await.unwrap();
        
        // Update the todo item
        let update_json = r#"{
            "id": 1,
            "title": "Updated Task",
            "content": "Updated Content",
            "priority": 2,
            "due_date": "2024-12-25 10:00:00"
        }"#;
        let result = update_todo_item_with_db(update_json, &db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "update_todo_item finish");
        
        // Verify the update
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, Some("Updated Task".to_string()));
        assert_eq!(items[0].content, Some("Updated Content".to_string()));
        assert_eq!(items[0].priority, Some(2));
    }

    #[tokio::test]
    async fn test_get_todo_items_by_category_id() {
        let db = create_test_db().await.unwrap();
        
        // Create todo items with different categories
        let item1_json = r#"{
            "title": "Category 1 Task",
            "category_id": 1,
            "due_date": "",
            "user_id": 1
        }"#;
        let item2_json = r#"{
            "title": "Category 2 Task",
            "category_id": 2,
            "due_date": "",
            "user_id": 1
        }"#;
        let item3_json = r#"{
            "title": "Another Category 1 Task",
            "category_id": 1,
            "due_date": "",
            "user_id": 1
        }"#;
        
        create_todo_item_with_db(item1_json, &db).await.unwrap();
        create_todo_item_with_db(item2_json, &db).await.unwrap();
        create_todo_item_with_db(item3_json, &db).await.unwrap();
        
        // Get items for category 1
        let result = get_todo_items_by_category_id_with_db(1, &db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| item.category_id == Some(1)));
        
        // Get items for category 2
        let result = get_todo_items_by_category_id_with_db(2, &db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].category_id, Some(2));
    }

    #[tokio::test]
    async fn test_delete_todo_item() {
        let db = create_test_db().await.unwrap();
        
        // Create todo items
        let item_json = r#"{
            "title": "Task to Delete",
            "due_date": "",
            "user_id": 1
        }"#;
        create_todo_item_with_db(item_json, &db).await.unwrap();
        create_todo_item_with_db(item_json, &db).await.unwrap();
        
        // Verify both items exist
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 2);
        
        // Delete one item
        let result = delete_todo_item_with_db(1, &db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "delete_todo_item finish");
        
        // Verify only one item remains
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, 2);
    }

    #[tokio::test]
    async fn test_delete_all_todo_items() {
        let db = create_test_db().await.unwrap();
        
        // Create multiple todo items
        let item_json = r#"{
            "title": "Task",
            "due_date": "",
            "user_id": 1
        }"#;
        create_todo_item_with_db(item_json, &db).await.unwrap();
        create_todo_item_with_db(item_json, &db).await.unwrap();
        create_todo_item_with_db(item_json, &db).await.unwrap();
        
        // Verify items exist
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 3);
        
        // Delete all items
        let result = delete_all_todo_items_with_db(&db).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "delete_all_todo_items finish");
        
        // Verify no items remain
        let result = get_all_todo_items_with_db(&db).await.unwrap();
        let items: Vec<ResponseTodoItem> = serde_json::from_str(&result).unwrap();
        assert_eq!(items.len(), 0);
    }

    #[tokio::test]
    async fn test_todo_item_json_serialization() {
        let item = ResponseTodoItem {
            id: 1,
            title: Some("Test Task".to_string()),
            content: Some("Test Content".to_string()),
            link: Some("https://example.com".to_string()),
            color: Some("red".to_string()),
            priority: Some(1),
            due_date: "2024-12-31 23:59:59".to_string(),
            category_id: Some(1),
            status: Some(0),
            user_id: 1,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };
        
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("Test Task"));
        assert!(json.contains("Test Content"));
        assert!(json.contains("https://example.com"));
        
        // Test deserialization of input data
        let input = r#"{
            "title": "New Task",
            "content": "New Content",
            "due_date": "2024-12-31 23:59:59",
            "user_id": 1
        }"#;
        let parsed: TodoItemForInsert = serde_json::from_str(input).unwrap();
        assert_eq!(parsed.title, Some("New Task".to_string()));
        assert_eq!(parsed.content, Some("New Content".to_string()));
        assert_eq!(parsed.user_id, 1);
    }
}