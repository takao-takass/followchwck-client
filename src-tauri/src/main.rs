#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
    println!("received: WEEEEEEEEEEI!");
    println!("received: WEEEEEEEEEEI!");
    println!("received: WEEEEEEEEEEI!");
    println!("received: WEEEEEEEEEEI!");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}


#[tauri::command]
fn login(mail: String, password: String) -> Result<String, String> {

    println!("Do login action.");
    println!("Mailaddress: {}", &mail);
    println!("Password: {}", &password);

    // Login action
    if &password.len() > &0 {
        Ok("okokokk".into())
    } else {
        Err("errororor".into())
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
