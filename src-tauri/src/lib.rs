// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn comando_custom(mivariable: String) -> String {
    let texto_formateado = format!("Invocado desde JS, {}", mivariable);
    texto_formateado
}

#[tauri::command]
fn otro_comando() {
    println!("Otro comando")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![comando_custom, otro_comando])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
