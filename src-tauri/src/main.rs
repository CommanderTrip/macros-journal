// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod macros;
mod user_details;

use macros::macro_breakdown;
use user_details::{ActivityLevel, Gender, UserDetails, WeightGoal};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



fn main() {
    let mut user = UserDetails::new(
        67,
        194,
        25,
        Gender::Male,
        ActivityLevel::Sedentary,
        WeightGoal::LoseWeight,
    );
    println!("Your daily Calories: {} ", user.get_recommended_calories());
    println!(
        "Your daily macro breakdown:
        Protein: {:?} grams/ {:?} Calories
        Fat: {:?} grams/ {:?} Calories
        Carbohydrates: {:?} grams/ {:?} Calories",
        macro_breakdown(&mut user).protein[0],
             macro_breakdown(&mut user).protein[1],
             macro_breakdown(&mut user).fat[0],
             macro_breakdown(&mut user).fat[1],
             macro_breakdown(&mut user).carbohydrates[0],
             macro_breakdown(&mut user).carbohydrates[1],
    );

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
