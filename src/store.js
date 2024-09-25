// store.js
import { reactive } from 'vue'

export const steps = Object.freeze({
    GREET: 0,
    YOU: 1,
    REE: 2,
    TDEE: 3,
    GOAL: 4,
    CALORIES: 5,
    MACROS: 6
})

export const store = reactive({
    index: steps.GREET,
    units: "",
    age: 0
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