<template>
<div class="home border flex-column">
  <router-link class="button" :to="{name: 'login'}">Login</router-link>
  <router-link class="button" :to="{name: 'signup'}">Signup</router-link>
</div>
</template>

<style lang="scss" scoped>
.home {
  width: 20em;
  background-color: aliceblue;
  margin: 5em auto 0 auto;
}

.button {
  margin: 2em auto;
  width: 10em;
}
</style>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import {verifySessionWithoutRedirect} from '../client';

@Component({})
export default class TaskView extends Vue {
  private mounted() {
    verifySessionWithoutRedirect()
        .then(() => this.$router.push({name: 'home'}))
        .catch();
  }
}
</script>