<template>
<div class="container flex-column">
  Title
  <input class="row" type="text" v-model="article.title" placeholder="Title"/>
  Text
  <textarea class="row" v-model="article.text" placeholder="Text"/>
  Identifier
  <input class="row" type="text" v-model="article.id" placeholder="Short Title"/>
  Tags
  <input class="row" type="text" v-model="tags" placeholder="Tags"/>
  <button v-on:click="update">Update</button>
</div>
</template>

<script lang="ts">
import {Component, Vue, Prop} from 'vue-property-decorator';
import Article, {newArticle, setTags, getTags} from '@/models/Article';
import {getArticle, updateArticle} from '@/client';

@Component
export default class EditArticle extends Vue {
  private title: string = '';
  private article: Article = newArticle();
  private tags: string = '';

  private mounted() {
    const title = this.$route.params['title'];
    getArticle(title).then((res) => {
        this.article = res.data;
        this.tags = getTags(this.article);
    });
  }

  private update() {
    setTags(this.article, this.tags);
    updateArticle(this.article).then(() => {
      this.$router.push({name: 'show-article', params: {title: this.article.id}});
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