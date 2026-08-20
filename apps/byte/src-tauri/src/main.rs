// Trên Windows, bản release KHÔNG được mở kèm cửa sổ console đen phía sau.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    byte_app_lib::chay()
}
