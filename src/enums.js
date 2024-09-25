export const steps = Object.freeze({
    GREET: 0,
    YOU: 1,
    REE: 2,
    TDEE: 3,
    GOAL: 4,
    CALORIES: 5,
    MACROS: 6
})

export const ReeFormula = Object.freeze({
    MifflinStJeor: Symbol(), // The more accepted one
    HarrisAndBenedict: Symbol()
})

export const Gender = Object.freeze({
    Male: Symbol(),
    Female: Symbol()
})

export const Units = Object.freeze({
    Imperial: Symbol(),
    Metric: Symbol(),
})

export const ActivityLevel = Object.freeze({
    Sedentary: Symbol(), // Everyday activities like walking, a couple of flights of stairs, eating, talking, etc.
    LightActivity: Symbol(), // Any activity that burns an additional 200-400 calories for females or 250-500 calories for males.
    ModerateActivity: Symbol(), // Any activity that burns an additional 400-650 calories for females or 500-800 calories for males.
    VeryActive: Symbol(), // Any activity that burns an additional 650+ calories for females or 800+ calories for males.
})

export const WeightGoal = Object.freeze({
    LoseWeight: Symbol(),
    MaintainWeight: Symbol(),
    GainWeight: Symbol()
})