<script setup lang="ts">

import {store} from "../../store";
import {Gender, Units} from "../../enums";
</script>

<template>
  <fieldset class="you">
    <legend>Who...are you?</legend>

    <section class="stack">
      <label for="age">Age</label>
      <input
          name="age"
          type="number"
          min="0"
          max="100"
          v-model="store.age"
          placeholder="years"
      />
    </section>

    <section>
      <label>Gender</label>
      <div>
        <input type="radio" id="male" value="{{Gender.Male}}" v-model="store.gender"/>
        <label for="male">Male</label>
      </div>
      <div>
        <input type="radio" id="female" value="{{Gender.Female}}" v-model="store.gender"/>
        <label for="female">Female</label>
      </div>
    </section>

    <section>
      <label>Height</label>
      <div class="stack" v-if="store.units === Units.Imperial">
        <input
            type="number"
            min="0"
            max="1000"
            placeholder="feet"
        />
        <input type="number"
               min="0"
               max="11"
               placeholder="inches"/>
      </div>
      <div v-else>
        <input type="number"
               min="0"
               max="1000"
               placeholder="centimeters"/>
      </div>
    </section>

    <section>
      <label for="weight">Weight</label>
      <div v-if="store.units === Units.Imperial">
        <input
            name="weight"
            type="number"
            min="0"
            max="1000"
            placeholder="pounds"
        />
      </div>
      <div v-else>
        <input
            name="weight"
            type="number"
            placeholder="kilograms"/>
      </div>
    </section>
  </fieldset>
</template>

<style scoped lang="scss">
.you {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
}

.stack {
  display: flex;
  flex-direction: column;
}

.you section {
  max-width: 33%;
}

input {
  width: 5rem;
}
</style>