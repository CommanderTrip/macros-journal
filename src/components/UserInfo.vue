<script setup>
import {store} from "../store.js";
import PreferredUnits from "./UserInfoSections/PreferredUnits.vue";
import PersonalComposition from "./UserInfoSections/PersonalComposition.vue";


</script>

<template>
  <div>
    <h1>Your Measurements</h1>
    <PreferredUnits/>
    <PersonalComposition/>
  </div>
</template>

<style>

</style>