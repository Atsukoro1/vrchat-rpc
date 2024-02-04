use tauri::command;

#[command] 
pub fn open_socket_handle() -> bool {
    true
}

#[command]
pub fn close_socket_handle() -> bool {
    true
}