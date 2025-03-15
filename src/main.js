const { invoke } = window.__TAURI__.core;

let input;
let msg;

function crear_directorio() {
  const nombre = "MiBlocDeNotas";

  try {
    invoke("crear_directorio", { nombre: nombre });
  } catch (error) {  }
}
crear_directorio();

// Se ejecuta sin bloquear otro codigo
async function listarArchivos() {
  msg.innerHTML = "";  // Limpiar el contenido actual
  try {
    const archivos = await invoke("listar_archivos_en_carpeta", {});
    console.log("Archivos recibidos:", archivos);  // Verificación en la consola

    // Crear una nueva lista de archivos sin la extensión .txt
    const archivosSinExtension = archivos.map(archivo => {
      // Eliminar la extensión .txt
      const nombreSinExtension = archivo.replace('.txt', '');
      // Crear el enlace con el nombre sin la extensión
      return `<a href="tarjeta.html#${nombreSinExtension}" class="tarjeta">${nombreSinExtension}</a>`;
    });

    // Unir los archivos con saltos de línea (<br>) en lugar de comas
    const archivosSinSaltos = archivosSinExtension.join("<br>");  // Usar <br> para saltos de línea en HTML

    // Insertar los archivos como HTML
    msg.innerHTML += archivosSinSaltos;

  } catch (error) {
    alert("Error al listar archivos", error);
  }
}

async function insertar_tarjeta() {
  const titulo = input.value;
  const contenido = "";
  try {
    await invoke("crear_nota", { titulo: titulo, contenido: contenido });
  } catch (error) {
    alert("Error al crear archivo");
  }
  listarArchivos();
  input.value = "";
}

window.addEventListener("DOMContentLoaded", () => {
  input = document.querySelector("#greet-input");
  msg = document.querySelector("#output");
  listarArchivos();

  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    if (input.value === "") {
      alert("Por favor, escribe algo en el campo de texto.");
      return;
    }
    insertar_tarjeta();
  });
});


let urlCompleta = window.location.href; // Devuelve la URL completa de la página actual
let fragmento = window.location.hash; // Devuelve el fragmento de la URL, incluyendo el signo #
