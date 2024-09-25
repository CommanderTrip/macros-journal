<script setup lang="ts">

import {store, steps} from "../store";
import {s} from "vite/dist/node/types.d-aGj9QkWt.js";
</script>

<template>
  <nav>
    <div class="nav-item" :class="{'selected': store.index == steps.YOU}">
      <span class="material-symbols-outlined">person</span>
      <lable>You</lable>
    </div>
    <div class="nav-item" :class="{'selected': store.index == steps.REE }">
      <span class="material-symbols-outlined">hotel</span>
      <label>REE</label>
    </div>
    <div class="nav-item" :class="{'selected': store.index == steps.TDEE }">
      <span class="material-symbols-outlined">exercise</span>
      <label>TDEE</label>
    </div>
    <div class="nav-item" :class="{'selected': store.index == steps.GOAL }">
      <span class="material-symbols-outlined">flag</span>
      <label>Goal</label>
    </div>
    <div class="nav-item" :class="{'selected': store.index == steps.CALORIES }">
      <span class="material-symbols-outlined">calculate</span>
      <label>Calories</label>
    </div>
    <div class="nav-item" :class="{'selected': store.index == steps.MACROS }">
      <span class="material-symbols-outlined">donut_large</span>
      <label>Macros</label>
    </div>
  </nav>
</template>

<style scoped lang="scss">
nav {
  position: sticky;
  top: 0;
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-bottom: 0.5rem;
  padding-block: 1rem;
  border-bottom: 2px solid black;
  background: #1E90FF;
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

.selected {
  color: darkred;
  border: 2px solid red;
  border-radius: 10px;
}
</style>