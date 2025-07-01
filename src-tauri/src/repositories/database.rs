use sea_orm::*;
use std::path::PathBuf;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
  // アプリのディレクトリ内にデータベースを作成
  let db_path = get_data_directory().await.map_err(|e| {
    log::error!("Failed to get data directory: {}", e);
    DbErr::Custom("Failed to get data directory".to_string())
  })?;
  let db_url = format!("sqlite://{}/solo.db", db_path);
  
  log::info!("Attempting to connect to database at: {}", db_url);

  // データベースに接続
  let db = Database::connect(&db_url).await.map_err(|e| {
    log::error!("Failed to connect to database at {}: {}", db_url, e);
    e
  })?;
  Ok(db)
}

// データベースのパスを取得する関数
async fn get_data_directory() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  if cfg!(debug_assertions) {
    // デバッグモードでは従来通り
    let path = PathBuf::from("target/debug/data");
    Ok(path.to_string_lossy().to_string())
  } else {
    // リリースモードでは適切なアプリデータディレクトリを使用
    use std::fs;
    
    // アプリの実行ファイルからの相対パスを試す
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Failed to get executable directory")?;
    
    // macOSアプリバンドル内のResourcesディレクトリを探す
    let resources_data = exe_dir.join("../Resources/data");
    if resources_data.exists() {
      log::info!("Using macOS app bundle resources path: {:?}", resources_data);
      return Ok(resources_data.to_string_lossy().to_string());
    }
    
    // リソースディレクトリの別のパターンを試す
    let app_data = exe_dir.join("data");
    if app_data.exists() {
      log::info!("Using executable directory data path: {:?}", app_data);
      return Ok(app_data.to_string_lossy().to_string());
    }
    
    // データディレクトリが見つからない場合は作成
    log::warn!("Data directory not found, creating new one at: {:?}", app_data);
    fs::create_dir_all(&app_data)?;
    
    // 元のデータベースファイルがある場合はコピー
    let fallback_db = PathBuf::from("data/solo.db");
    let target_db = app_data.join("solo.db");
    if fallback_db.exists() && !target_db.exists() {
      log::info!("Copying database from fallback location");
      fs::copy(&fallback_db, &target_db)?;
    }
    
    Ok(app_data.to_string_lossy().to_string())
  }
}
