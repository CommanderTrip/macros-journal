<script setup>
import {watch, ref} from "vue";
import {store} from "../store.js";

// const age = ref(0);
//
// watch(age, (newAge, oldAge) => {
//     console.log(oldAge);
//     console.log(newAge);
// });

</script>

<template>
  <div>
    <h1>Your Measurements</h1>
    <div>
        <fieldset>
            <legend>What units would you prefer?</legend>

            <div>
                <input type="radio" id="imperial" name="units" value="imperial" v-model="store.units"/>
                <label for="imperial">Imperial</label>
            </div>
     
            <div>
                <input type="radio" id="metric" name="units" value="metric" v-model="store.units"/>
                <label for="metric">Metric</label>   
            </div>
        </fieldset>

      <fieldset>
        <legend>Who...are you?</legend>
        <label for="age">Age</label>
        <input name="age" type="number" min="0" max="100" v-model="store.age" />

        <div v-if="store.units === 'imperial'">
          <p>imperial</p>
        </div>
        <div v-else>
          <p>metric</p>
        </div>




      </fieldset>

    </div>
  </div>
</template>