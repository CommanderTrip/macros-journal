// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod main_calculators;
mod conversions;

use main_calculators::calculate_ree;
use conversions::{pounds_to_kilograms, feet_inches_to_centimeters};

fn main() {
    // FIXME: Cmds arent binding
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate_ree])
        .invoke_handler(tauri::generate_handler![pounds_to_kilograms])
        .invoke_handler(tauri::generate_handler![feet_inches_to_centimeters])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
