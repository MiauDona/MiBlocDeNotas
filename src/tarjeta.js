const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", async () => {
  const titleElement = document.querySelector("#title");
  const contentElement = document.querySelector("#content");

  // Obtener el título de la nota desde la URL
  let titulo = decodeURIComponent(window.location.hash.substring(1));
  titleElement.textContent = titulo;

  // Obtener el contenido del archivo desde el backend
  try {
    let contenido = await invoke("leer_archivo", { titulo: titulo });
    contentElement.textContent = contenido;
  } catch (error) {
    contentElement.textContent = "(Nota vacía o error al cargar)";
  }

  // Hacer el contenido editable al hacer clic
  contentElement.addEventListener("click", () => {
    if (contentElement.querySelector("textarea")) return;
    
    const textarea = document.createElement("textarea");
    textarea.value = contentElement.textContent;
    textarea.classList.add("editable-textarea");

    contentElement.textContent = "";
    contentElement.appendChild(textarea);
    textarea.focus();

    // Guardar cambios al perder el foco
    textarea.addEventListener("blur", async () => {
      let nuevoContenido = textarea.value;
      contentElement.textContent = nuevoContenido;

      try {
        await invoke("crear_nota", { titulo: titulo, contenido: nuevoContenido });
      } catch (error) {
        alert("Error al guardar la nota: " + error);
      }
    });
  });
});

