<script setup lang="ts">

import {store} from "../store";
import {Units} from "../enums";
import {ref} from "vue";

const imperialWeight = ref(null);

function convertAndStoreImperialWeight() {
  store.weight = (imperialWeight.value * 0.4535924).toFixed(0);
}
</script>

<template>
  <section class="stack">
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
          min="0"
          max="1000"
          placeholder="kilograms"
          v-model="store.weight"
      />
    </div>
  </section>
</template>

<style scoped lang="scss">
section {
  input {
    width: 100%;
  }
}

.stack {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>