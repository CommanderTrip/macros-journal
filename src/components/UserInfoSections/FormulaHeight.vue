<script setup lang="ts">

import {store} from "../../store";
import {Units} from "../../enums";
import {reactive} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const imperialHeight = reactive({
  feet: null,
  inches: null
});

function convertAndStoreImperialHeight() {
  let feet = Number(imperialHeight.feet)
  let inches = Number(imperialHeight.inches)

  invoke('feet_inches_to_centimeters', {
    feet,
    inches
  }).then(message => {
    store.height = Number(message)
  })
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
          @change="convertAndStoreImperialHeight"
      />
      <input type="number"
             min="0"
             max="11"
             placeholder="inches"
             v-model="imperialHeight.inches"
             @change="convertAndStoreImperialHeight"
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