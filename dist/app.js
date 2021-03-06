function print_command(){
    window.__TAURI__
        .invoke('print_command')
}

function rev_string_command(){
    var display_Msg = document.getElementById('text-msg');
    var rev_display_Msg = document.getElementById('rev-text-msg');
    var Msg = document.getElementById('text_area').value;
    display_Msg.textContent = Msg;

    window.__TAURI__
        .invoke('rev_string_command', { s: Msg})
        .then(rev_s => {
            console.log(rev_s);
            rev_display_Msg.textContent = rev_s;
        })
}

function chat_command(){
    var chat_name = document.getElementById("name_text_area").value;
    var chat_msg = document.getElementById("msg_text_area").value;
    var display_lv = document.getElementById("display_lv");
    var display_msg = document.getElementById("display_msg");
    var lv = 0;
    if(document.getElementById("display_lv").textContent != null){
        lv = Number(document.getElementById("display_lv").textContent);
    }

    window.__TAURI__
        .invoke('chat_command',
            { text: {
                name: chat_name,
                lv: lv,
                message: chat_msg
            }})
        .then(text => {
            display_lv.textContent = text.lv;
            display_msg.textContent = text.message;
        })
}

function age_command(){
    var age = parseInt(document.getElementById("age_text_area").value);
    var display_age = document.getElementById("display_age");
    console.log(age);
    if(isNaN(age)){
        display_age.textContent = "Error: 多分数字じゃないです";
    }else{
    window.__TAURI__
        .invoke('age_command', { age })
        .then(age_class => {
            console.log('age_command', age_class)
            display_age.textContent = age_class;
        })
        .catch(e => {
            console.error('age_command',e)
            display_age.textContent = e;
        })
    }
}

function file_dialog_command (query) {
    let display_zip_path = document.getElementById("display_zip_path");
    let display_unzip_path = document.getElementById("display_unzip_path");

    if(query == "zip"){
        window.__TAURI__.dialog
            .open({recursive: true, multiple: true}).then(files => display_zip_path.textContent = files);
    }else if(query == "unzip"){
        window.__TAURI__.dialog
            .open({filters: {extensions: ['.zip']},recursive: true }).then(files => display_unzip_path.textContent = files);
    }
}

function zip_command(){
    let zip_path = document.getElementById("display_zip_path").textContent;
    let display_zip_result = document.getElementById("display_zip_result");
    let zip_output_path_value =  document.getElementById("zip_output_path_text_area").value;
    if(zip_output_path_value == ""){
        zip_output_path_value = "./"
    }
    if(zip_path == ""){
        display_zip_result.textContent = "ファイルを選択してください";
    }else{
        console.log(zip_path);
        window.__TAURI__
            .invoke("zip_command", {filepath: zip_path, outputpath: zip_output_path_value})
            .then(result => {
                console.log(result);
                display_zip_result.textContent = result
        });
    }
}

function unzip_command(){
    let unzip_path = document.getElementById("display_unzip_path").textContent;
    let display_zip_result = document.getElementById("display_unzip_result");
    if(unzip_path == ""){
        display_zip_result.textContent = "ZIPファイルを選択してください";
    }else{
        console.log(unzip_path);
        window.__TAURI__
            .invoke("unzip_command", {filename: unzip_path})
            .then(result => {
                console.log(result);
                display_zip_result.textContent = result
        });
    }
}

function ask_command() {
    let hangman = "＿＿＿＿＿＿＿<br>　　&nbsp;||　　<br>　Λ||Λ<br>（&nbsp;/&nbsp;⌒ヽ<br>　|&nbsp;|&nbsp;　　|<br>　∪&nbsp;亅|<br>　&nbsp;|　|　|<br>　&nbsp;∪∪<br>　‐ニ三ニ‐";
    let display_hangman = document.getElementById("display_hangman");
    window.__TAURI__.dialog
        .ask("あなたは｜好きですか？", "")
        .then(ans => {
            if(ans){
                display_hangman.innerHTML = hangman;
            }
        });
}

window.__TAURI__.window
    .appWindow.listen('tauri://move', ({ event, payload }) => {
            const { x, y } = payload // payload here is a `PhysicalPosition`
            console.log('x', x);
            console.log('y', y);
        })
    
function emitMessage_command() {
    window.__TAURI__.event
        .emit('front-to-back', "hello from front")
}