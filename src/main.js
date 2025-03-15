const { invoke } = window.__TAURI__.core;

let input;
let msg;

// Se ejecuta sin bloquear otro codigo
async function comando_custom() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  const result = await invoke("comando_custom", { mivariable: input.value }); // Invoca el comando_custom y manda a mivariable (rust) el valor del input (js)
  msg.textContent = result;
}

window.addEventListener("DOMContentLoaded", () => {
  input = document.querySelector("#greet-input");
  msg = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    comando_custom();
  });
});
