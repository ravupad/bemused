<template>
  <LoginComponent/>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import LoginComponent from '@/components/Login.vue';

@Component({
  components: {
    LoginComponent,
  },
})
export default class Login extends Vue {
}
</script>

<style scoped>
.login {
  width: 400px;
  margin: 50px auto;
  padding: 20px;
  background-color: aliceblue;
  font-size: 1.2em;
}

.login >>> .flex-row {
  margin: 20px 0;
}

.login >>> .flex-row > .label {
  width: 100px;
}

.login >>> .flex-row > input {
  width: 200px;
  padding: 10px;
}

.login >>> button {
  margin: 20px 0;
}
</style>