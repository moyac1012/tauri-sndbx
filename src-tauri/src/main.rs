use tauri::Manager;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    name: String,
    lv: u32,
    message:String
}

#[tauri::command]
fn print_command() {
  println!("Call from JavaScript");
}

#[tauri::command]
fn rev_string_command(s : String) -> String {
  let rev_s: String = s.chars().rev().collect();
  rev_s
}

#[tauri::command]
fn chat_command(text: ChatMessage) -> ChatMessage {
  let username = text.name.clone();
  ChatMessage {
        name: text.name,
        lv: text.lv + 1,
        message: format!("{} : {}", username, text.message)
    }
}

#[tauri::command]
fn age_command(age: i32) -> Result<String, String>{
  match age {
      0..=19 => Ok(format!("未成年")),
      20..=125 => Ok(format!("成人")),
      126.. => Err(format!("死んでいます")),
      _ => Err(format!("生まれてません")),
  }
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
          app.listen_global("front-to-back", |event| {
          println!(
              "got front-to-back with payload {:?}",
              event.payload().unwrap()
          )
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      print_command,
      rev_string_command,
      chat_command,
      age_command,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
