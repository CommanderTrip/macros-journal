#[derive(Clone, Copy)]
pub struct UserDetails {
    height: u16,
    pub weight: u16,
    age: u8,
    gender: Gender,
    activity_level: ActivityLevel,
    weight_goal: WeightGoal,
    pub ree_formula: ReeFormula,
    units: Units,
    recommended_calories: u16,
}

impl UserDetails {
    pub fn new(
        height: u16,
        weight: u16,
        age: u8,
        gender: Gender,
        activity_level: ActivityLevel,
        weight_goal: WeightGoal,
    ) -> UserDetails {
        return UserDetails {
            // User input needed
            height: height,
            weight: weight,
            age: age,
            gender: gender,
            activity_level: activity_level,
            weight_goal: weight_goal,

            // Defaults
            ree_formula: ReeFormula::MifflinStJeor,
            units: Units::Imperial,
            recommended_calories: 0,
        };
    }

    pub fn get_recommended_calories(&mut self) -> u16 {
        if self.recommended_calories == 0 {
            self.calculate_recommended_calories();
        }
        return self.recommended_calories;
    }

    /**
     * Resting Energy Expenditure (AKA Resting Metabolic Rate)
     * The calories burned when your body is completely at rest.
     * Resource - https://blog.nasm.org/nutrition/resting-metabolic-rate-how-to-calculate-and-improve-yours
     * TODO: Change this to not use floating point values
     */
    fn calculate_ree(self) -> u16 {
        let mut height = self.height as f32;
        let mut weight = self.weight as f32;

        if self.units == Units::Imperial {
            // Convert height and weight to metric for the calculation
            height = height as f32 * 2.54; // in Centimeters
            weight = weight as f32 * 0.4535924; // in Kilograms
        }

        match self.ree_formula {
            ReeFormula::MifflinStJeor => match self.gender {
                Gender::Male => {
                    (10.0 * weight + 6.25 * height - 5.0 * self.age as f32 + 5.0) as u16
                }
                Gender::Female => {
                    (10.0 * weight + 6.25 * height - 5.0 * self.age as f32 - 161.0) as u16
                }
            },
            ReeFormula::HarrisAndBenedict => match self.gender {
                Gender::Male => {
                    (88.362 + 13.397 * weight + 4.799 * height - 5.677 * self.age as f32) as u16
                }
                Gender::Female => {
                    (447.593 + 9.247 * weight + 3.098 * height - 4.330 * self.age as f32) as u16
                }
            },
        }
    }

    fn calculate_recommended_calories(&mut self) {
        let daily_calories = self.calculate_tdee();
        match self.weight_goal {
            WeightGoal::LoseWeight => {
                self.recommended_calories = daily_calories - (daily_calories as f32 * 0.2) as u16
            }
            WeightGoal::MaintainWeight => self.recommended_calories = daily_calories,
            WeightGoal::GainWeight => {
                self.recommended_calories = daily_calories + (daily_calories as f32 * 0.2) as u16
            }
        }
    }

    /**
     * Total Daily Energy Expenditure
     * The number of calories your particular body burns in a day based on activity level.
     */
    fn calculate_tdee(&mut self) -> u16 {
        match self.activity_level {
            ActivityLevel::Sedentary => (self.calculate_ree() as f32 * 1.2) as u16,
            ActivityLevel::LightActivity => (self.calculate_ree() as f32 * 1.375) as u16,
            ActivityLevel::ModerateActivity => (self.calculate_ree() as f32 * 1.55) as u16,
            ActivityLevel::VeryActive => (self.calculate_ree() as f32 * 1.725) as u16,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ReeFormula {
    MifflinStJeor,
    HarrisAndBenedict,
}

#[derive(Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

#[derive(PartialEq, Clone, Copy)]
enum Units {
    Imperial,
    Metric,
}

#[derive(Clone, Copy)]
pub enum ActivityLevel {
    Sedentary, // Everyday activities like walking, a couple of flights of stairs, eating, talking, etc.
    LightActivity, // Any activity that burns an additional 200-400 calories for females or 250-500 calories for males.
    ModerateActivity, // Any activity that burns an additional 400-650 calories for females or 500-800 calories for males.
    VeryActive, // Any activity that burns an additional 650+ calories for females or 800+ calories for males.
}

#[derive(Clone, Copy)]
pub enum WeightGoal {
    LoseWeight,
    MaintainWeight,
    GainWeight,
}
