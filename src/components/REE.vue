<script setup lang="ts">
import {store} from "../store";
import {Gender, ReeFormula} from "../enums";
</script>

<template>
  <fieldset>
    <legend>Resting Energy Expenditure</legend>

    <section>
      <label>Which REE Formula would you like to use?</label>

      <div>
        <input
            id="MifflinStJeor"
            name="ree-formula"
            type="radio"
            value="{{ReeFormula.MifflinStJeor}}"
            v-model="store.reeFormula"
        />
        <label for="MifflinStJeor">Mifflin-St Jeor</label>
        <input
            id="HarrisAndBenedict"
            name="ree-formula"
            type="radio"
            value="{{ReeFormula.HarrisAndBenedict}}"
            v-model="store.reeFormula"
        />
        <label for="HarrisAndBenedict">Harris And Benedict</label>
      </div>

      <div v-if="store.reeFormula === ReeFormula.MifflinStJeor && store.gender === Gender.Male">
        <em>10 * {{ store.weight }} + 6.25 {{ store.height }} - 5 * {{ store.age }} + 5</em>
      </div>
      <div v-if="store.reeFormula === ReeFormula.MifflinStJeor && store.gender === Gender.Female">
        <em>10 * {{ store.weight }} + 6.25 {{ store.height }} - 5 * {{ store.age }} - 161</em>
      </div>
      <div v-if="store.reeFormula === ReeFormula.HarrisAndBenedict && store.gender === Gender.Male">
        <em>88.362 + 13.397 * {{ store.weight }} + 4.799 * {{ store.height }} - 5.677 * {{ store.age }}</em>
      </div>
      <div v-if="store.reeFormula === ReeFormula.HarrisAndBenedict && store.gender === Gender.Female">
        <em>447.593 + 9.247 * {{ store.weight }} + 3.098 * {{ store.height }} - 4.330 * {{ store.age }}</em>
      </div>
    </section>
  </fieldset>
</template>name="ree-formula"

<style scoped lang="scss">

</style>