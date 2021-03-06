mod ziplib;

use tauri::{ SystemTrayEvent, WindowBuilder, Manager, Position, PhysicalPosition};
use serde::{ Serialize, Deserialize };

use tauri::SystemTray;

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
  let username: String = text.name.clone();
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

#[tauri::command]
fn zip_command(filepaths: Vec<&str>, outputpath: String) -> String{
  let mut zip_result_msg: String = String::new();
  for filepath in filepaths{
    match ziplib::create_zip(filepath, outputpath.clone()) {
      Ok(zip_filename) => {
        println!("[Success] File written to {:?}", zip_filename);
        zip_result_msg += &format!("[Success] File written to {:?}", zip_filename);
      }
      Err(e) => {
          eprintln!("Error: {:?}", e);
          zip_result_msg += &format!("Error: {:?}", e);
      }
    }
  }
  zip_result_msg
}

#[tauri::command]
fn unzip_command(filename : &str) -> String {
  match ziplib::extract_zip(filename) {
    Ok(unzip_msg) => {
      println!("[Success] File written to {}", unzip_msg);
      format!("[Success] File written to {}", unzip_msg)
    }
    Err(e) => {
        eprintln!("Error: {:?}", e);
        format!("Error: {:?}", e)
    }
  }
}

fn main(){
  let tray = SystemTray::new();
    tauri::Builder::default()
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: pos,
        size: _,
        ..
      } => {
        let menu_window = app.get_window("menu");
        let physical_pos:PhysicalPosition<i32> = {PhysicalPosition{x: pos.x as i32, y: pos.y as i32}};
        let physical_pos = Position::Physical(physical_pos);

        match menu_window {
          Some(window) =>{
            if window.is_visible().unwrap(){
              window.hide().unwrap();
            }else{
              window.set_position(physical_pos).unwrap();
              window.show().unwrap();
            }
          },
          None => {
            let window_x_size = 400.0;
            let window_y_size = 300.0;
            let _window = WindowBuilder::new(
              app,
              "menu".to_string(),
              tauri::WindowUrl::App("menu.html".into()),
            )
            .always_on_top(true)
            .decorations(false)
            .inner_size(window_x_size, window_y_size)
            .min_inner_size(window_x_size, window_y_size)
            .max_inner_size(window_x_size, window_y_size)
            .visible(false)
            .position(pos.x, pos.y)
            .build();
          }
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      print_command,
      rev_string_command,
      chat_command,
      zip_command,
      age_command,
      unzip_command
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
