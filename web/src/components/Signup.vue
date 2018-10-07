<template>
<div class="signup border flex-column">
  <h4>Signup</h4>
  <div class="error" v-if="error != ''">{{error}}</div>
  <div class="flex-row">
    <div class="label">Username</div>
    <input type="text" class="border" v-model="username">
  </div>
  <div class="flex-row">
    <div class="label">Password</div>
    <input type="password" class="border" v-model="password" v-on:keyup.enter="signup">
  </div>
  <button class="button" v-on:click="signup">Signup</button>
</div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import {signup} from '@/client.ts';

@Component
export default class Login extends Vue {
  private error: string = '';
  private username: string = '';
  private password: string = '';

  private signup() {
    signup(this.username, this.password)
        .then(() => {
          this.$router.push({name: 'login'});
        })
        .catch((error) => {
          this.error = error.response.data.message;
        });
  }
}
</script>
