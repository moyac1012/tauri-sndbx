
window.__TAURI__.event
    .listen("tauri://file-drop-hover", ({event, payload}) => {
        console.log("file-drop-hover", payload);
        
    })

window.__TAURI__.event
    .listen("tauri://file-drop-cancelled", ({event, payload}) => {
        console.log("file-drop-cancelled", payload);
    })

window.__TAURI__.event
    .listen("tauri://file-drop", ({event, payload}) => {
        let display_files = document.getElementById("display_files");
        let text = "";
        payload.forEach(element => {
            text += element;
            text += "\n";
        });
        display_files.textContent = text;
    })
