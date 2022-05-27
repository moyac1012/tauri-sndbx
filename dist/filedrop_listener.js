const sleep = waitTime => new Promise( resolve => setTimeout(resolve, waitTime) );

function zip_command(filepaths){
    let zip_output_path_value =  document.getElementById("zip_output_path_text_area").textContent;
    window.__TAURI__
        .invoke("zip_command", {filepaths: filepaths, outputpath: zip_output_path_value})
        .then(result => {
            console.log(result);
            display_zip_result.textContent = result
    });
}

async function file_dialog_command (query) {
    let dialog_btn = document.getElementById("dialog_btn");
    dialog_btn.style.pointerEvents = "none";

    let zip_output_path_text_area = document.getElementById("zip_output_path_text_area");
    window.__TAURI__.dialog
        .open({recursive: true, multiple: true, directory: true})
        .then(files => {
            console.log(files);
            if(files != null){
                zip_output_path_text_area.textContent = files;
            }
        });
    await sleep(1000);
    dialog_btn.style.pointerEvents = "auto";
    
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
        zip_command(payload);
    })

window.__TAURI__.path
    .documentDir().then(DocDir => document.getElementById("zip_output_path_text_area").textContent = DocDir+"zip_data");