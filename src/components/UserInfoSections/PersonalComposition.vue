<script setup lang="ts">

import {store} from "../../store";
import {Gender, Units} from "../../enums";
import {reactive, ref} from "vue";

const imperialWeight = ref(null);
const imperialHeight = reactive({
  feet: null,
  inches: null
});

function convertAndStoreImperialMajorHeight() {
  store.height = imperialHeight.feet * 30.48
  convertAndStoreImperialMinorHeight()
}

function convertAndStoreImperialMinorHeight() {
  if (imperialHeight.inches === null) return
  store.height = store.height + imperialHeight.inches * 2.54
}

function convertAndStoreImperialWeight() {
  store.weight = imperialWeight.value * 0.4535924;
}

// TODO: Add validation that all input as a value
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
        <input
            type="radio"
            id="male"
            :value="Gender.Male"
            v-model="store.gender"
        />
        <label for="male">Male</label>
      </div>
      <div>
        <input
            type="radio"
            id="female"
            :value="Gender.Female"
            v-model="store.gender"
        />
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
            v-model="imperialHeight.feet"
            @change="convertAndStoreImperialMajorHeight"
        />
        <input type="number"
               min="0"
               max="11"
               placeholder="inches"
               v-model="imperialHeight.inches"
               @change="convertAndStoreImperialMinorHeight"
        />
      </div>
      <div v-else>
        <input type="number"
               min="0"
               max="1000"
               placeholder="centimeters"
               v-model="store.height"
        />
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
            v-model="imperialWeight"
            @change="convertAndStoreImperialWeight"
        />
      </div>
      <div v-else>
        <input
            name="weight"
            type="number"
            placeholder="kilograms"
            v-model="store.weight"
        />
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