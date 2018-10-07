<template>
<div class="login border flex-column">
  <h4>Login</h4>
  <div class="error" v-if="error != ''">{{error}}</div>
  <div class="flex-row">
    <div class="label">Username</div>
    <input type="text" v-model="username" class="border">
  </div>
  <div class="flex-row">
    <div class="label">Password</div>
    <input type="password" v-model="password" class="border" v-on:keyup.enter="login">
  </div>
  <button class="button" v-on:click="login">Login</button>
</div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import {login, verifySessionWithoutRedirect} from '../client';

@Component
export default class Login extends Vue {
  private error: string = '';
  private username: string = '';
  private password: string = '';

  private mounted() {
    verifySessionWithoutRedirect()
        .then(() => this.$router.push('/home'))
        .catch();
  }

  private login() {
    login(this.username, this.password)
        .then(() => {
          this.$router.push({name: 'home'});
        })
        .catch((error: any) => {
          this.error = error.response.data.message;
        });
  }
}
</script>
