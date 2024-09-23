<script setup>

import {store} from "./store.js";

import ProgressionNav from "./components/ProgressionNav.vue";
import REE from "./components/REE.vue";
import TDEE from "./components/TDEE.vue";
import Goal from "./components/Goal.vue";
import Calories from "./components/Calories.vue";
import Macros from "./components/Macros.vue";

function resetStore() {
  store.index = 0
}
</script>

<template>
  <main>
    <ProgressionNav />
    <RouterView />
  </main>
  <div v-if="$route.fullPath === '/'">
    <div class="welcome">
      <h1>Macronutrients Calculator</h1>
      <p>Index {{store.index}}</p>
      <button @click="store.index++">Let's get started!</button>
    </div>
    <REE class="transition-in" v-if="store.index > 0"/>
    <TDEE class="transition-in" v-if="store.index > 1"/>
    <Goal class="transition-in" v-if="store.index > 2"/>
    <Calories class="transition-in" v-if="store.index > 3"/>
    <Macros class="transition-in" v-if="store.index > 4"/>

  </div>
  <footer>
    <button @click="resetStore">restart</button>
  </footer>
</template>

<style>
html {
  width: 100%;
  height: 100%;
  background-image: linear-gradient(-45deg, #1E90FF, #00CED1);
}

body {
  margin-inline: 2rem;
}

main {
  display: flex;
  flex-direction: column;
  margin-inline: 15%;
}
</style>

<style scoped>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
footer {
  position: absolute;
  bottom: 0;
}

.welcome {
  display: flex;
  flex-direction: column;
  align-items: center;

  button {
    font-size: large;
  }
}

.transition-in {
  position: absolute;
  bottom: -100px;
  animation: fadeInAndMoveUp 1s forwards;
  animation-timing-function: ease-in;
}

@keyframes fadeInAndMoveUp {
  from {
    opacity: 0;
    position: relative;
  }
  to {
    position: relative;
    bottom: 0;
    opacity: 1;
  }
}
</style>
