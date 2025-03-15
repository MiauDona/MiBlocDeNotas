use directories::UserDirs;
use std::env;
use std::fs;
use std::fs::create_dir;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn insertar_tarjeta(titulo: String, enlace: String) -> String {
    return format!(
        "<a class='tarjeta' href='tarjeta.html#{}'> {} </a>",
        enlace, titulo
    );
}

#[tauri::command]
fn leer_archivo(path: String) -> Result<String, String> {
    // Abrir el archivo
    let mut archivo = match File::open(&path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Error al abrir el archivo: {}", e)),
    };

    let mut contenido = String::new();
    // Leer el archivo y almacenar su contenido en `contenido`
    match archivo.read_to_string(&mut contenido) {
        Ok(_) => Ok(contenido),
        Err(e) => Err(format!("Error al leer el archivo: {}", e)),
    }
}

#[tauri::command]
fn crear_nota(titulo: &str, contenido: &str) -> String {
    let path = obtener_ruta_carpeta() + "\\" + &titulo + ".txt"; // Ruta en Windows
    match File::create(&path) {
        Ok(mut archivo) => {
            if let Err(e) = archivo.write_all(contenido.as_bytes()) {
                return format!("Error al escribir en el archivo: {}", e);
            }
            format!("{}, creado en {}", titulo, path)
        }
        Err(e) => format!("Error al crear el archivo: {}", e),
    }
}

fn obtener_ruta_usuario() -> String {
    // Obtener la ruta del directorio personal del usuario
    let user_home = env::var("USERPROFILE").unwrap_or_else(|_| env::var("HOME").unwrap_or_default());

    // Retorna la ruta
    user_home
}

fn obtener_ruta_carpeta() -> String {
    // Obtener la ruta del directorio personal del usuario
    let user_home = env::var("USERPROFILE").unwrap_or_else(|_| env::var("HOME").unwrap_or_default());

    // Construir la ruta completa para la carpeta
    let path = Path::new(&user_home).join("MiBlocDeNotas");

    // Retorna la ruta
    path.to_string_lossy().to_string()
}


#[tauri::command]
fn crear_directorio(nombre: &str) -> String {
    // Obtener el directorio del usuario
    if let Some(user_dirs) = UserDirs::new() {
        let user_home = user_dirs.home_dir();
        
        // Construir la ruta completa para el nuevo directorio
        let path = user_home.join(nombre);

        // Intentar crear el directorio
        match create_dir(&path) {
            Ok(_) => format!("Directorio '{}' creado en: {}", nombre, path.display()),
            Err(e) => format!("Error al crear el directorio: {}", e),
        }
    } else {
        "Error al obtener el directorio del usuario".to_string()
    }
}

#[tauri::command]
fn listar_archivos_en_carpeta() -> Result<Vec<String>, String> {
    let ruta_usuario = obtener_ruta_usuario().to_string() + "\\MiBlocDeNotas";
    let dir = Path::new(&ruta_usuario);

    // Verifica si la ruta es un directorio
    if !dir.is_dir() {
        return Err("La ruta proporcionada no es un directorio".to_string());
    }

    let mut archivos: Vec<String> = Vec::new();

    // Lee el contenido del directorio
    match fs::read_dir(dir) {
        Ok(entradas) => {
            for entrada in entradas {
                match entrada {
                    Ok(entry) => {
                        let nombre = entry
                            .file_name()
                            .into_string()
                            .unwrap_or_else(|_| String::from("Nombre no válido"));
                        
                        archivos.push(nombre);
                    }
                    Err(e) => {
                        return Err(format!("Error al leer una entrada del directorio: {}", e))
                    }
                }
            }
            Ok(archivos)
        }
        Err(e) => Err(format!("Error al leer el directorio: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            insertar_tarjeta,
            crear_nota,
            crear_directorio,
            leer_archivo,
            listar_archivos_en_carpeta
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
