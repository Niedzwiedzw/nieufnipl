<template>
  <div class="ArticleContainer">
    <ArticleMiniature v-for="article in articles" :key="article.date" :article="article"/>
  </div>
</template>

<script lang="ts">
    import {Component, Vue} from 'vue-property-decorator';
    import ArticleMiniature from '@/components/ArticleMiniature.vue';

    import {allArticles, Article} from '@/common';

    @Component({
        components: {
            ArticleMiniature,
        },
    })
    export default class ArticleContainer extends Vue {
        private articles: Article[] = [];

        protected async beforeMount() {
            this.articles = await allArticles();
        }

    }
</script>

<style scoped lang="scss">
  @import "../styles/common";

  .ArticleContainer {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax($minimal-miniature-width, 1fr));
    grid-auto-rows: $article-miniature-height;
    grid-gap: 1rem;
    grid-area: articles;
    justify-content: center;
  }
</style>
