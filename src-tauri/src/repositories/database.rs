use sea_orm::*;
use std::path::PathBuf;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
  // アプリのディレクトリ内にデータベースを作成
  let db_path = get_data_directory();
  let db_url = format!("sqlite://{}", db_path + "/solo.db");

  // データベースに接続
  let db = Database::connect(&db_url).await?;
  Ok(db)
}

// データベースのパスを取得する関数
fn get_data_directory() -> String {
  let path = PathBuf::from(if cfg!(debug_assertions) { "target/debug/data" } else { "data" });
  match path.to_str() {
    Some(path_str) => path_str.to_string(),
    None => "".to_string(),
  }
}
