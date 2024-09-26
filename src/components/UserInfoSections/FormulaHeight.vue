<script setup lang="ts">

import {store} from "../../store";
import {Units} from "../../enums";
import {reactive} from "vue";

const imperialHeight = reactive({
  feet: null,
  inches: null
});

function convertAndStoreImperialMajorHeight() {
  store.height = (imperialHeight.feet * 30.48).toFixed(0)
  convertAndStoreImperialMinorHeight()
}

function convertAndStoreImperialMinorHeight() {
  if (imperialHeight.inches === null) return
  store.height = (store.height + imperialHeight.inches * 2.54).toFixed(0)
}
</script>

<template>
  <section class="stack">
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
    <input v-else
           id="metric-height"
           type="number"
           min="0"
           max="1000"
           placeholder="centimeters"
           v-model="store.height"
    />

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

#metric-height {
  width: 6rem;
}
</style>