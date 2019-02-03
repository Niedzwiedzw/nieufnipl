<template>
  <div class="home">
    <ArticleContainer/>
    <!--<Sidebar/>-->
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
  @import "../styles/navbar";


  $sidebar-height: 25rem;

  .home {
    display: grid;
    height: calc(100vh - #{$navbar-height});
    grid-template-columns: 1fr $site-width 1fr;
    grid-template-rows: 1fr;
    grid-template-areas:
        "left articles right";

    grid-gap: 1.5rem;
  }

  @media only screen and (max-width: $site-width) {
    .home {
      grid-template-rows: 1fr;
      grid-template-columns: 1fr;
      grid-template-areas:
          "articles"
    }
  }
</style>
