<script>
import {watch, ref} from "vue";

const age = ref(0);

watch(age, (newAge, oldAge) => {
    console.log(oldAge);
    console.log(newAge);
});

</script>

<template>
    <h1>Your Measurements</h1>
    <div>
        <fieldset>
            <legend>What units would you prefer?</legend>

            <div>
                <input type="radio" id="imperial" name="units" value="imperial"/>
                <label for="imperial">Imperial</label>
            </div>
     
            <div>
                <input type="radio" id="metric" name="units" value="metric"/>
                <label for="metric">Metric</label>   
            </div>
        </fieldset>

      <fieldset>
        <legend>Who...are you?</legend>
        <input type="number" min="0" max="100" placeholder="Age" :value="age" required/>

      </fieldset>

    </div>
</template>