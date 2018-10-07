<template>
<div class="container flex-column">
  <input class="row" type="text" v-model="newArticle.title" placeholder="Title"/>
  <textarea class="row" v-model="newArticle.text" placeholder="Text"/>
  <input class="row" type="text" v-model="newArticle.id" placeholder="Short Title"/>
  <input class="row" type="text" v-model="tags" placeholder="Tags"/>
  <button v-on:click="createOrUpdate">Submit</button>
</div>
</template>

<script lang="ts">
import {Component, Vue, Prop} from 'vue-property-decorator';
import Article, {newArticle, setTags} from '@/models/Article';
import {createArticle, updateArticle} from '@/client';

@Component
export default class NewArticle extends Vue {
  private newArticle: Article = newArticle();
  private tags: string = '';

  private mounted() {
    const title = this.$route.params['title'];
  }

  private createOrUpdate() {
    setTags(this.newArticle, this.tags);
    createArticle(this.newArticle).then(() => {
      this.$router.push({name: 'article', params: {title: this.newArticle.id}});
    });
  }
}
</script>

<style scoped="true">
.container {
  width: 400px;
  margin: 0 auto;
}
.row {width: 100%;}
</style>