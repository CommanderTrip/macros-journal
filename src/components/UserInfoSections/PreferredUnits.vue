<script setup lang="ts">

import {store} from "../../store";
import {Units} from "../../enums";
</script>

<template>
  <fieldset>
    <!-- TODO: Style to not look like simple radio -->
    <legend>What units would you prefer?</legend>
    <input
        type="radio"
        id="imperial"
        name="units"
        :value="Units.Imperial"
        v-model="store.units"
    />
    <label for="imperial">Imperial</label>
    <input
        type="radio"
        id="metric"
        name="units"
        :value="Units.Metric"
        v-model="store.units"
    />
    <label for="metric">Metric</label>
  </fieldset>
</template>

<style scoped lang="scss">

</style>