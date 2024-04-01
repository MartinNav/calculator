const { invoke } = window.__TAURI__.tauri;

document.getElementById("submitInput").addEventListener("click", function() {
    const userInput = document.getElementById("userInput").value;
    if (userInput.trim() === "") {
        alert("Please input something");
        return;
    }

    invoke('call_parser', { input: userInput })
        .then((result) => {
            document.getElementById("parserOutput").innerText = `output: ${result}`;
        })
        .catch((error) => {
            console.error('Error calling parser:', error);
        });
});

