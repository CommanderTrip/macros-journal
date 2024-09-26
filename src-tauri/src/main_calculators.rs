/**
 * Calculates Resting Energy Expenditure with dependency injection.
 * Units will already be converted to metric.
 * Values will be inherently multiplied by 1000 to force no floating point values.
 */
#[tauri::command]
pub fn calculate_ree(gender: &str, formula: &str, weight: u64, height: u64, age: u64) -> u64 {
    match formula {
        "MifflinStJeor" => match gender {
            "male" => {
                10000 * weight + 6250 * height - 5000 * age + 5000
            }
            "female" => {
                if 10000 * weight + 6250 * height - 5000 * age < 161000 {
                    return 0;
                }
                10000 * weight + 6250 * height - 5000 * age - 161000
            }
            _ => 0
        },
        "HarrisAndBenedict" => match gender {
            "male" => {
                88362 + 13397 * weight + 4799 * height - 5677 * age
            }
            "female" => {
                447593 + 9247 * weight + 3098 * height - 4330 * age
            }
            _ => 0
        },
        _ => 0
    }
}

mod test {
    use crate::main_calculators::calculate_ree;

    #[test]
    fn calculate_ree_male_recc_min() {
        let result = calculate_ree("male", "MifflinStJeor", 0, 0, 0);
        assert_eq!(result, 5000);
    }

    #[test]
    fn calculate_ree_male_recc_real() {
        // 250 lbs, 6' 5", 30
        let result = calculate_ree("male", "MifflinStJeor", 113398, 195580, 30);
        assert_eq!(result, 2356210000);
    }

    // FIXME
    #[test]
    fn calculate_ree_male_recc_max() {
        let result = calculate_ree("male", "MifflinStJeor", 0, 0, 0);
        assert_eq!(result, 5000);
    }

    #[test]
    fn calculate_ree_male_unrecc_min() {
        let result = calculate_ree("male", "HarrisAndBenedict", 0, 0, 0);
        assert_eq!(result, 88362);
    }

    #[test]
    // 250 lbs, 6' 5", 30
    fn calculate_ree_male_unrecc_real() {
        let result = calculate_ree("male", "HarrisAndBenedict", 113398, 195580, 30);
        assert_eq!(result, 2457699478);
    }

    // FIXME
    #[test]
    fn calculate_ree_male_unrecc_max() {
        let result = calculate_ree("male", "HarrisAndBenedict", 0, 0, 0);
        assert_eq!(result, 88362);
    }

    #[test]
    fn calculate_ree_female_recc_min() {
        let result = calculate_ree("female", "MifflinStJeor", 0, 0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn calculate_ree_female_recc_real() {
        // 140lbs, 5' 5", 18
        let result = calculate_ree("female", "MifflinStJeor", 63502, 165100, 18);
        assert_eq!(result, 1666644000);
    }

    // FIXME
    #[test]
    fn calculate_ree_female_recc_max() {
        let result = calculate_ree("female", "MifflinStJeor", 0, 0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn calculate_ree_female_unrecc_min() {
        let result = calculate_ree("female", "HarrisAndBenedict", 0, 0, 0);
        assert_eq!(result, 447593);
    }

    #[test]
    fn calculate_ree_female_unrecc_real() {
        let result = calculate_ree("female", "HarrisAndBenedict", 63502, 165100, 18);
        assert_eq!(result, 1099052447);
    }

    // FIXME
    #[test]
    fn calculate_ree_female_unrecc_max() {
        let result = calculate_ree("female", "HarrisAndBenedict", 0, 0, 0);
        assert_eq!(result, 447593);
    }
}