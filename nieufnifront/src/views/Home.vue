<template>
  <div class="home">
    <ArticleContainer/>
    <Sidebar/>
  </div>
</template>

<script lang="ts">
    import { Component, Vue } from 'vue-property-decorator';

    import ArticleMiniature from '@/components/ArticleMiniature.vue';
    import ArticleContainer from '@/components/ArticleContainer.vue';
    import Sidebar from '@/components/Sidebar.vue';

    @Component({
        components: {
            ArticleMiniature,
            ArticleContainer,
            Sidebar,
        },
    })
    export default class Home extends Vue {}
</script>
<style lang="scss" scoped>
  @import "../styles/common";

  $sidebar-height: 25rem;

  .home {
    display: grid;
    height: calc(100vh - calc(3rem + 4rem));
    grid-template-columns: 1fr $standard-minimal-width;
    grid-template-rows: 1fr $sidebar-height;
    grid-template-areas:
        "articles sidebar"
        "articles sidebar";

    grid-gap: 1.5rem;
  }

  @media only screen and (max-width: $mobile-threshold) {
    .home {
      grid-template-rows: $sidebar-height 1fr;
      grid-template-areas:
          "sidebar sidebar"
          "articles articles"
    }
  }
</style>
