<template>
  <router-link class="ArticleMiniature hoverable" tag="div" :to="{name: 'singlearticle', params: {articleId: article.id}}">
    <div class="img-container">
      <img src="https://i.ytimg.com/vi/W9t6GZ0vNPA/hqdefault.jpg" alt="miniatura">
    </div>
    <h3 class="title">Artykul #{{article.id}}</h3>
    <p class="short-text">Lorem ipsum dolor sit amet, consectetur adipisicing elit...</p>
  </router-link>
</template>

<script lang="ts">
    import {Component, Prop, Vue} from 'vue-property-decorator';
    import {Article, getArticle} from '@/common';

    @Component({
        components: {},
    })
    export default class ArticleMiniature extends Vue {
        @Prop() protected article!: Article;

        protected async beforeMount() {
            this.article = await getArticle(this.article.id);
        }
    }
</script>

<style scoped lang="scss">
  @import "../styles/common.scss";

  .title {
    grid-area: title;
    margin-left: 1rem;
  }

  .short-text {
    grid-area: text;

    margin: 1rem;
  }

  .img-container {
    width: 100%;
    height: 100%;
    grid-area: image;
    img {
      object-fit: cover;
      width: $article-miniature-height;
      height: $article-miniature-height;
    }
  }

  .ArticleMiniature {
    border: 2px solid black;
    display: grid;
    grid-template-areas:
        "image title"
        "image text";
    grid-template-columns: $article-miniature-height 1fr;
    grid-template-rows: 4rem 1fr;
  }
</style>
