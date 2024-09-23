<script setup>

import { RouterLink } from 'vue-router';

</script>

<template>
    <div>
        <h1>Macro Calculator</h1>
        <p>Breaking down your macros comes down to 5 steps:</p>
        <ol>
            <li>Calculating your Resting Energy Expenditure (REE)</li>
            <li>Calculating your Total Daily Energy Expenditure (TDEE)</li>
            <li>Applying your fitness goals</li>
            <li>Breaking down your recommended daily Calories</li>
            <li>Breaking down your recommended <em>Macronutrients</em> from the Calories</li>
        </ol>
        <div>
            <p>Let's start by first getting your measurements!</p>
            <RouterLink to="/user"><button>Next</button></RouterLink>
        </div>
    </div>
</template>

<style scoped lang="scss">

</style>