<script setup lang="ts">

import {store} from "../../store";
</script>

<template>
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