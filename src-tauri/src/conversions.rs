#[tauri::command]
pub fn pounds_to_kilograms(mut weight: u32) -> u32 {
    // Returns the weight as an integer that includes 3 decimal places at a max of 453.592 kg
    if weight > 1000 { weight = 1000; }
    (weight as f64 * 453.59237) as u32
}

#[tauri::command]
pub fn feet_inches_to_centimeters(mut feet: u32, mut inches: u32) -> u32 {
    // Returns the weight as an integer that includes 3 decimal place at a max of 30507.940 cm
    if feet > 1000 { feet = 1000; }
    if inches > 11 { inches = 11; }
    feet * 30480 + inches * 2540
}

mod test {
    use crate::conversions::{feet_inches_to_centimeters, pounds_to_kilograms};

    #[test]
    fn lbs2kg_lower_bound() {
        let result = pounds_to_kilograms(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn lbs2kg_realistic() {
        let result = pounds_to_kilograms(220);
        assert_eq!(result, 99790); // 220 lbs = 99.790 kg
    }

    #[test]
    fn lbs2kg_mid() {
        let result = pounds_to_kilograms(9468780);
        assert_eq!(result, 453592); // 150 lbs = 68.038 kg
    }

    #[test]
    fn lbs2kg_upper_bound() {
        let result = pounds_to_kilograms(4294967295);
        assert_eq!(result, 453592);
    }

    #[test]
    fn fi2cm_lower_bound() {
        let result = feet_inches_to_centimeters(0, 0);
        assert_eq!(0, 0);
    }

    #[test]
    fn fi2cm_realistic() {
        let result = feet_inches_to_centimeters(6, 5);
        assert_eq!(result, 195580); // 6ft 5in = 195.580 cm
    }

    #[test]
    fn fi2cm_upper() {
        let result = feet_inches_to_centimeters(4294967295, 4294967295);
        assert_eq!(result, 30507940); // Over the max value of u32
    }
}