
const invoke = window.__TAURI__.invoke;

function command(){
    invoke('my_custom_command');
}
