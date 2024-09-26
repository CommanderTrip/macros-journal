<script setup lang="ts">

import {store} from "../store";
import {Gender, Units} from "../enums";
</script>

<template>
  <fieldset>
    <legend>Preface</legend>

    <section>
      <label>Preferred Units</label>
      <div class="input-selection">
        <div :class="{'selected': store.units === Units.Imperial}">
          <input
              type="radio"
              id="imperial"
              name="units"
              :value="Units.Imperial"
              v-model="store.units"
          />
          <label for="imperial">Imperial</label>
        </div>

        <div :class="{'selected': store.units === Units.Metric}">
          <input
              type="radio"
              id="metric"
              name="units"
              :value="Units.Metric"
              v-model="store.units"
          />
          <label for="metric">Metric</label>
        </div>
      </div>
      <em>Imperial units will be converted to metric</em>
    </section>

    <section>
      <label>Gender</label>
      <div class="input-selection">
        <div :class="{'selected': store.gender === Gender.Male}">
          <input
              type="radio"
              id="male"
              :value="Gender.Male"
              v-model="store.gender"
          />
          <label for="male">Male</label>
        </div>
        <div :class="{'selected': store.gender === Gender.Female}">
          <input
              type="radio"
              id="female"
              :value="Gender.Female"
              v-model="store.gender"
          />
          <label for="female">Female</label>
        </div>
      </div>
    </section>
  </fieldset>
</template>

<style scoped lang="scss">
fieldset {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
}

section {
  display: flex;
  flex-direction: column;
  align-items: center;

  label {
    font-size: 20px;
  }

  > label {
    font-size: 24px;
  }
}

input {
  margin: 0;
  appearance: none;
}

.input-selection {
  display: flex;
  border: 2px solid black;
  border-radius: 4px;

  div {
    padding: 5px 5px;
  }
}

.selected {
  background-color: royalblue;
}
</style>