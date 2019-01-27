<template>
  <div class="SingleArticle">
    <p class="article-metadata">by {{article.author }} ~ {{article.date}}</p>
    <div class="article-body" v-html="article.rendered_text">
      <!--{{article.rendered_text}}-->
    </div>
  </div>
</template>

<script lang="ts">
    import {Component, Vue, Prop} from 'vue-property-decorator';
    import {Article, emptyArticle, getArticle} from '@/common';

    @Component({
        components: {},
    })
    export default class SingleArticle extends Vue {
        @Prop(String) protected articleId!: string;
        private article: Article = emptyArticle();

        protected async beforeMount() {
            this.article = await getArticle(this.articleId);
        }
    }
</script>

<style scoped lang="scss">
  @import "../styles/common";

  .SingleArticle {
    display: grid;
    grid-template-columns: 1fr $article-width 1fr;
    grid-template-rows: 3rem 1fr;
    grid-template-areas:
        "left top right"
        "left body right";
  }

  .article-metadata {
    grid-area: top
  }

  .article-body {
    grid-area: body
  }

  @media only screen and (max-width: $mobile-threshold) {
    .SingleArticle {
      grid-template-columns: 1fr;
      grid-template-areas:
          "top"
          "body";
    }
  }
</style>
