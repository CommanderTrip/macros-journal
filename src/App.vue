<script setup>

import {store, nextStep, steps} from "./store.js";

import ProgressionNav from "./components/ProgressionNav.vue";
import UserInfoView from "./components/UserInfo.vue";
import REE from "./components/REE.vue";
import TDEE from "./components/TDEE.vue";
import Goal from "./components/Goal.vue";
import Calories from "./components/Calories.vue";
import Macros from "./components/Macros.vue";
import GettingStarted from "./components/GettingStarted.vue";

function resetStore() {
  store.index = 0
}
</script>

<template>
  <ProgressionNav />
  <main>
    <div class="welcome">
      <h1>Macronutrients Calculator</h1>
      <GettingStarted />
      <p>Index {{store.index}}</p>
    </div>

    <UserInfoView class="transition-in" v-if="store.index > steps.YOU"/>
    <REE class="transition-in" v-if="store.index > steps.REE"/>
    <TDEE class="transition-in" v-if="store.index > steps.TDEE"/>
    <Goal class="transition-in" v-if="store.index > steps.GOAL"/>
    <Calories class="transition-in" v-if="store.index > steps.CALORIES"/>
    <Macros class="transition-in" v-if="store.index > steps.MACROS"/>

    <button @click="nextStep(store.index)">Next</button>
  </main>

  <footer>
    <button @click="resetStore">restart</button>
  </footer>
</template>

<style>
html {
  width: 100vw;
  min-height: 100vh;
  background: linear-gradient(-45deg, #1E90FF, #00CED1);
  background-size: cover;
}

body {
  margin: 0 ;
}

main {
  display: flex;
  min-height: 100vh;
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
  position: sticky;
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
  animation: fadeInAndMoveUp 0.7s forwards;
  animation-timing-function: ease-out;
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
