<template>
  <router-link class="ArticleMiniature hoverable" tag="div" :to="{name: 'singlearticle', params: {id: 1}}">
    <div class="img-container">
      <img src="https://i.ytimg.com/vi/W9t6GZ0vNPA/hqdefault.jpg" alt="miniatura">
    </div>
    <h3 class="title">Artykul # {{article.id}}</h3>
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
        @Prop(Number) protected articleId!: number;
        private article!: Article;

        protected beforeCreate() {
            this.article = getArticle(this.articleId);
        }
    }
</script>

<style scoped lang="scss">
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
      width: 100%;
      height: 100%;
    }
  }

  .ArticleMiniature {
    border: 2px solid black;
    display: grid;
    grid-template-areas:
        "image title"
        "image text";
    grid-template-columns: 10rem 1fr;
    grid-template-rows: 1.8rem 1fr;
  }
</style>
