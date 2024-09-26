<script setup lang="ts">
import {store} from "../store";
import {Gender, ReeFormula, Units} from "../enums";
import FormulaWeight from "./FormulaWeight.vue";
import FormulaHeight from "./UserInfoSections/FormulaHeight.vue";
import FormulaAge from "./UserInfoSections/FormulaAge.vue";
</script>

<template>
  <fieldset>
    <legend>Resting Energy Expenditure</legend>

    <section id="ree-choice">
      <label>Which REE Formula would you like to use?</label>

      <div class="input-selection">
        <div id="recommendedFormula" :class="{'selected': store.reeFormula === ReeFormula.MifflinStJeor}">
          <input
              id="MifflinStJeor"
              name="ree-formula"
              type="radio"
              :value="ReeFormula.MifflinStJeor"
              v-model="store.reeFormula"
          />
          <label for="MifflinStJeor">Mifflin-St Jeor</label>
          <em>recommended</em>
        </div>
        <div :class="{'selected': store.reeFormula === ReeFormula.HarrisAndBenedict}">
          <input
              id="HarrisAndBenedict"
              name="ree-formula"
              type="radio"
              :value="ReeFormula.HarrisAndBenedict"
              v-model="store.reeFormula"
          />
          <label for="HarrisAndBenedict">Harris and<br/>Benedict</label>
        </div>
      </div>
    </section>

    <div v-if="store.reeFormula === ReeFormula.MifflinStJeor" class="formula">
      <em>10 *</em>
      <FormulaWeight/>
      <em>+ 6.25 *</em>
      <FormulaHeight/>
      <em>- 5 *</em>
      <FormulaAge/>
      <em v-if="store.gender === Gender.Male">+ 5</em>
      <em v-else>-161</em>
    </div>
    <div v-if="store.reeFormula === ReeFormula.HarrisAndBenedict" class="formula">
      <em v-if="store.gender === Gender.Male">88.362 + 13.397 *</em>
      <em v-else>447.593 + 9.247 *</em>
      <FormulaWeight/>
      <em v-if="store.gender === Gender.Male">+ 4.799 *</em>
      <em v-else>+ 3.098 *</em>
      <FormulaHeight/>
      <em v-if="store.gender === Gender.Male">- 5.677 *</em>
      <em v-else>- 4.330 *</em>
      <FormulaAge/>
    </div>
  </fieldset>
</template>name="ree-formula"

<style scoped lang="scss">
section {
  display: flex;
  flex-direction: column;
  align-items: center;

  label {
    font-size: 20px;
    text-align: center;
  }

  > label {
    font-size: 24px;
  }
}

input {
  margin: 0;
  appearance: none;
}

.formula {
  display: flex;
  align-items: flex-end;
  justify-content: center;
  margin-block: 1rem 2rem;
  font-size: 16px;

  em {
    padding-inline: 0.75rem;
  }
}

.input-selection {
  display: flex;
  border: 2px solid black;
  border-radius: 4px;

  div {
    padding: 5px 5px;
    display: inherit;
    align-items: center;
  }
}

.selected {
  background-color: royalblue;
}

#recommendedFormula {
  display: flex;
  flex-direction: column;
  align-items: center;
}

#ree-choice {
  margin-top: 2rem;
}
</style>