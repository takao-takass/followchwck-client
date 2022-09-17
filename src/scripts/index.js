
const invoke = window.__TAURI__.invoke;

function command(){
    invoke('my_custom_command');
}

function login(){

    let mail = $("#mail").val();
    let password = $("#password").val();

    invokeParam = {
        mail : mail,
        password : password,
    }

    invoke('login', invokeParam)
        .then((message) => alert(message))
        .catch((error) => alert(error));
}