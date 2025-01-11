use encoding_rs;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::error::Error;
pub async fn read_csv_to_2d_array(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
  let contents = download_csv(file_path.to_string()).await;
  let mut result = Vec::new();
  match contents {
    Ok(contents) => {
      let mut first_line = true;
      for record in contents.lines() {
        // 1行目はヘッダなのでスキップ
        if first_line {
          first_line = false;
          continue;
        }
        let record: Vec<&str> = record.split(",").collect();
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        result.push(row);
      }
    }
    Err(e) => return Err(e.into()),
  }

  Ok(result)
}
async fn download_csv(url: String) -> Result<String, String> {
  // HTTPリクエストを送信
  let response = reqwest::get(&url).await.map_err(|e| format!("Request error: {}", e))?;

  if response.status().is_success() {
    // レスポンスボディをバイト列として取得
    let bytes = response.bytes().await.map_err(|e| format!("Failed to read response bytes: {}", e))?;

    // エンコーディングの自動検出を試みる
    let (decoded_string, _, has_errors) = encoding_rs::UTF_8.decode(&bytes);

    if has_errors {
      // UTF-8に問題がある場合、Shift_JISや他のエンコーディングを試す
      let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::SHIFT_JIS)) // 必要に応じて他のエンコーディングを指定
        .build(&bytes[..]);

      let mut text = String::new();
      std::io::Read::read_to_string(&mut decoder, &mut text).map_err(|e| format!("Failed to decode response text: {}", e))?;
      Ok(text)
    } else {
      Ok(decoded_string.to_string())
    }
  } else {
    Err(format!("Failed to fetch CSV: HTTP {}", response.status()))
  }
}
