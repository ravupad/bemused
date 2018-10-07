<template>
<div>
  <router-link :to="{name: 'new-article', params: {'title': ''}}">New Article</router-link>
  <div v-for="article in articles" v-bind:key="article.id" class="flex-row">
    <router-link :to="{name: 'show-article', params: {'title': article.id}}">
      {{article.title}}
    </router-link>
    <div>{{article.tags}}</div>
    <button v-on:click="() => deleteArticle(article)">Delete</button>
    <router-link :to="{name: 'edit-article', params: {'title': article.id}}">Edit</router-link>
  </div>
</div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import {deleteArticle, getArticles} from '../client';
import {ListViewArticle} from '../models/Article';

@Component
export default class Articles extends Vue {
  private articles: ListViewArticle[] = [];

  private mounted() {
    getArticles().then((res) => {
      this.articles = res.data;
    });
  }

  private deleteArticle(article: ListViewArticle) {
    deleteArticle(article.id).then(() => {
      this.articles = this.articles.filter((a) => a !== article);
    });
  }
}
</script>

<style scoped>
</style>