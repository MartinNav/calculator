const { invoke } = window.__TAURI__.tauri;

let display;

async function calculate() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  console.log("calculate called");
  display.value = await invoke("calculate", { equation: display.value });

  console.log("calculate finished");
 adjustFontSize(display);
}

window.addEventListener("DOMContentLoaded", () => {
  display= document.querySelector("#display");
  document.querySelector("#equals").addEventListener("click", (e) => {
   // e.preventDefault();
    calculate();
  });
});

