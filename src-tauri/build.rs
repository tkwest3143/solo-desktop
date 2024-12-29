use std::fs;
use std::path::PathBuf;
fn main() {
  tauri_build::build();
  copy_data_file("solo.db".to_string());
}

fn copy_data_file(file_name: String) {
  let mut path = PathBuf::from("data");
  path.push(file_name.clone());

  let contents = fs::read(&path).expect("Something went wrong reading the file");
  path.pop();
  path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  path.push("target/release/data");
  match fs::create_dir_all(&path) {
    Ok(_) => println!("ディレクトリが正常に作成されました: {}", path.display()),
    Err(e) => eprintln!("ディレクトリの作成中にエラーが発生しました: {}", e),
  }
  path.push(file_name.clone());
  fs::write(&path, contents.clone()).expect("Unable to write file");

  path.pop();
  path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  path.push("target/debug/data");
  match fs::create_dir_all(&path) {
    Ok(_) => println!("ディレクトリが正常に作成されました: {}", path.display()),
    Err(e) => eprintln!("ディレクトリの作成中にエラーが発生しました: {}", e),
  }
  path.push(file_name.clone());
  fs::write(&path, contents.clone()).expect("Unable to write file");
}
