<template>
<div v-html="article.text"></div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import Article, {newArticle} from '@/models/Article';
import {deleteArticle, getArticle} from '@/client';

@Component
export default class ArticleView extends Vue {
  private article: Article = newArticle();

  private mounted() {
    getArticle(this.$route.params['title']).then((article) => {
      this.article = article.data;
    });
  }

  private deleteArticle() {
    deleteArticle(this.article.id).then(() => {
      this.$router.push({name: 'articles'});
    });
  }
}
</script>
