// store.js
import {reactive} from 'vue'
import {ActivityLevel, Gender, ReeFormula, steps, Units, WeightGoal} from "./enums.js";

export const store = reactive({
    index: steps.GREET,
    units: Units.Imperial,
    age: null,
    weight: null,
    height: null,
    gender: Gender.Male,
    activityLevel: ActivityLevel.LightActivity,
    weightGoal: WeightGoal.LoseWeight,
    reeFormula: ReeFormula.MifflinStJeor,

    reeCalories: null
})

export const nextStep = (index) => {
    switch (index) {
        case steps.GREET:
            store.index = steps.YOU
            break
        case steps.YOU:
            store.index = steps.REE
            break
        case steps.REE:
            store.index = steps.TDEE
            break
        case steps.TDEE:
            store.index = steps.GOAL
            break
        case steps.GOAL:
            store.index = steps.CALORIES
            break
        case steps.CALORIES:
            store.index = steps.MACROS
            break
        default:
    }
}