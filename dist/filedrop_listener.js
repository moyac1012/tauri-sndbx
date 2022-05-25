function zip_command(filepaths){
    let zip_output_path_value =  document.getElementById("zip_output_path_text_area").value;
    if(zip_output_path_value == ""){
        zip_output_path_value = "./"
    }
    window.__TAURI__
        .invoke("zip_command", {filepaths: filepaths, outputpath: zip_output_path_value})
        .then(result => {
            console.log(result);
            display_zip_result.textContent = result
    });
}

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
        payload.forEach(filepath => {
            text += filepath;
            text += "\n";
        });
        display_files.textContent = text;

        zip_command(payload);
    })
