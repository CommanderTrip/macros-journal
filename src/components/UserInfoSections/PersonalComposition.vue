<script setup lang="ts">

import {store} from "../../store";
import {Gender, Units} from "../../enums";
import {reactive, ref} from "vue";


// TODO: Add validation that all input as a value
</script>

<template>
  <fieldset class="you">
    <legend>Who...are you?</legend>


  </fieldset>
</template>

<style scoped lang="scss">
.you {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
}


.you section {
  max-width: 33%;
}

input {
  width: 5rem;
}
</style>