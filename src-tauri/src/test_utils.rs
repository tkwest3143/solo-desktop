#[cfg(test)]
pub mod test_utils {
    use sea_orm::{Database, DbConn, DbErr};
    use std::sync::atomic::{AtomicU32, Ordering};
    
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    
    /// Create an in-memory SQLite database for testing
    pub async fn create_test_db() -> Result<DbConn, DbErr> {
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let db_url = format!("sqlite::memory:test_{}", counter);
        
        let db = Database::connect(&db_url).await?;
        setup_schema(&db).await?;
        Ok(db)
    }
    
    /// Setup database schema for testing
    async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
        use sea_orm::Statement;
        
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
        
        // Create work_settings table
        db.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"
            CREATE TABLE work_settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                start DATETIME NOT NULL,
                end DATETIME NOT NULL,
                rest_start DATETIME NOT NULL,
                rest_end DATETIME NOT NULL,
                memo TEXT,
                working_unit INTEGER NOT NULL,
                user_id INTEGER NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
            "#.to_string(),
        )).await?;
        
        // Create japanese_holidays table
        db.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"
            CREATE TABLE japanese_holidays (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
            "#.to_string(),
        )).await?;
        
        // Create todo_categories table
        db.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            r#"
            CREATE TABLE todo_categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                memo TEXT,
                color TEXT,
                user_id INTEGER NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
            "#.to_string(),
        )).await?;
        
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
}