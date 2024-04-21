const {invoke} = window.__TAURI__.tauri;

async function calculate() {
    display.value = await invoke("calculate", {equation: display.value})
        .then((result) => result)
        .catch((error) => {
            displaying_error = true;
            return error;
        });

    adjustFontSize();
}

window.addEventListener("DOMContentLoaded", () => {
    document.getElementById("equals").addEventListener("click", (_) => {
        void (calculate());
    });
});

