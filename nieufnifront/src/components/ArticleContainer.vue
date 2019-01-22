<template>
  <div class="ArticleContainer">
    <ArticleMiniature v-for="articleId in 20" :id="articleId" :key="articleId"/>
  </div>
  
</template>

<script lang="ts">
    import {Component, Vue} from 'vue-property-decorator';
    import ArticleMiniature from '@/components/ArticleMiniature.vue';

    @Component({
        components: {
            ArticleMiniature,
        },
    })
    export default class ArticleContainer extends Vue {
    }
</script>

<style scoped lang="scss">
  @import "../styles/common";

  .ArticleContainer {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax($standard-minimal-width, 1fr));
    grid-auto-rows: 8rem;
    grid-gap: 1rem;

    grid-area: articles;
  }
</style>
