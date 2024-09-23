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

        <form @submit.prevent="">

            <input type="number" min="0" max="100" placeholder="Age" :value="age" required/>
            
        </form>
    </div>
</template>