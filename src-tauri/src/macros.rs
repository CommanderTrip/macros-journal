use crate::user_details::UserDetails;

/**
 * User Macros broken down into [Grams, Calories]
 */
#[derive(Debug)]
pub struct Macros {
    pub protein: [u16; 2],
    pub fat: [u16; 2],
    pub carbohydrates: [u16; 2],
}

/**
 * How much of each macronutrient to should consume.
 */
pub fn macro_breakdown(user: &mut UserDetails) -> Macros {
    let protein = protein_breakdown(&user);
    let fat = fat_breakdown(user);
    return Macros {
        protein: protein,
        fat: fat,
        carbohydrates: carbohydrate_breakdown(user, protein[1] + fat[1]),
    };
}

/**
 * How much protein to consume.
 * If the user is already lean, 1 gram of protein per pound of body weight is the typical rule of thumb. However,
 * a more balanced measure for users with moderate fat is 0.825 grams per pound. Users with excess fat should use
 * 0.65 grams per pound.
 * Note: 1g of protein = 4 Calories
 */
fn protein_breakdown(user: &UserDetails) -> [u16; 2] {
    let grams_of_protein = (user.weight as f32 * 0.85) as u16;
    return [grams_of_protein, grams_of_protein * 4];
}

/**
 * How much fat to consume.
 * Recommended Calories to be 20% - 30% of total TDEE, so 25% for a balance.
 * Note: 1g of Fat = 9 Calories
 */
fn fat_breakdown(user: &mut UserDetails) -> [u16; 2] {
    let fat_calories = (user.get_recommended_calories() as f32 * 0.25) as u16;
    return [fat_calories / 9, fat_calories];
}

/**
 * How much carborhydrate to consume.
 * 1g of Carbs = 4 Calories
 */
fn carbohydrate_breakdown(user: &mut UserDetails, allocated_calories: u16) -> [u16; 2] {
    let carb_calories = user.get_recommended_calories() - allocated_calories;
    return [carb_calories / 4, carb_calories];
}
