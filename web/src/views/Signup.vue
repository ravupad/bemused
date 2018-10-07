<template>
<SignupComponent/>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import SignupComponent from '@/components/Signup.vue';

@Component({
  components: {
    SignupComponent,
  },
})
export default class Signup extends Vue {
}
</script>


<style scoped>
.signup {
  width: 400px;
  margin: 50px auto;
  padding: 20px;
  background-color: aliceblue;
  font-size: 1.2em;
}

.signup >>> .flex-row {
  margin: 20px 0;
}

.signup >>> .flex-row > .label {
  width: 100px;
}

.signup >>> .flex-row > input {
  width: 200px;
  padding: 10px;
}

.signup >>> button {
  margin: 20px 0;
}
</style>