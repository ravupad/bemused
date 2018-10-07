<template>
<div class="home flex-column border">
  <router-link class="row button" :to="{name: 'task'}">Tasks</router-link>
  <router-link class="row button" :to="{name: 'list-article'}">Articles</router-link>
  <button class="row button" v-on:click="logout">Logout</button>
</div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import {verifySession, clearSession} from '@/client.ts';

@Component
export default class Login extends Vue {
  private mounted() {
    verifySession()
        .catch(() => {
          this.$router.push({name: 'login'});
        });
  }

  private logout() {
    clearSession()
        .then(() => {
          this.$router.push({name: 'start'});
        });
  }
}
</script>

<style scoped>
.home {
  width: 20em;
  margin: 50px auto;
  padding: 0 10px;
  background-color: aliceblue;
}

.row {
  margin: 30px 0;
  width: 10em;
}
</style>