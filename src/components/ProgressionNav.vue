<script setup lang="ts">

import { RouterLink } from 'vue-router';

</script>

<template>
  <nav>
    <div class="nav-item">
      <span class="material-symbols-outlined">hotel</span>
      <label>REE</label>
    </div>
    <div class="nav-item">
      <span class="material-symbols-outlined">exercise</span>
      <label>TDEE</label>
    </div>
    <div class="nav-item">
      <span class="material-symbols-outlined">flag</span>
      <label>Goal</label>
    </div>
    <div class="nav-item">
      <span class="material-symbols-outlined">calculate</span>
      <label>Calories</label>
    </div>
    <div class="nav-item">
      <span class="material-symbols-outlined">donut_large</span>
      <label>Macros</label>
    </div>
  </nav>
</template>

<style scoped lang="scss">
nav {
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-block: 2rem;
  border-bottom: 2px solid black;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.material-symbols-outlined {
  font-size: 64px;
  margin-inline: 2rem;
}
</style>