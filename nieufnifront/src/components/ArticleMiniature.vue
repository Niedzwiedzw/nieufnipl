<template>
  <router-link class="ArticleMiniature hoverable"
               tag="div"
               :to="{name: 'singlearticle', params: {articleId: article.id}}"
  >
    <div class="img-container">
      <img :src="article.image" alt="miniatura">
    </div>
    <h3 class="title">{{article.title}}</h3>
    <p class="short-text">{{ articleSummary }}</p>
  </router-link>
</template>

<script lang="ts">
    import {Component, Prop, Vue} from 'vue-property-decorator';
    import {Article, getArticle, articleSummary} from '@/common';

    @Component({
        components: {},
    })
    export default class ArticleMiniature extends Vue {
        @Prop() protected article!: Article;
        private articleSummary: string = '';

        protected async beforeMount() {
            this.article = await getArticle(this.article.id);
            this.articleSummary = articleSummary(this.article);
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
    margin-left: 1rem;
    margin-right: 1rem;
    margin-top: 0;
  }

  .img-container {
    width: 100%;
    height: 100%;
    grid-area: image;
    overflow: hidden;
    img {
      object-fit: cover;
      width: $article-miniature-height;
      height: $article-miniature-height;
    }
  }

  .ArticleMiniature {
    overflow: hidden;
    border: 2px solid lightgrey;
    border-radius: 1rem;
    display: grid;
    grid-template-areas:
        "image title"
        "image text";
    grid-template-columns: $article-miniature-height 1fr;
    grid-template-rows: 4rem 1fr;

    box-shadow: lightgrey 0.5rem 0.5rem 0.5rem;
  }
</style>
