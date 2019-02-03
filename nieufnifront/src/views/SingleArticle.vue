<template>
  <div class="SingleArticle">
    <p class="article-metadata">by {{ article.author }} ~ {{ creationTime }}</p>
    <div class="article-body" v-html="article.rendered_text">
      <!--{{article.rendered_text}}-->
    </div>

    <div id="disqus_thread"></div>
  </div>
</template>

<script lang="ts">
    import { Component, Vue, Prop } from 'vue-property-decorator';
    import { Article, emptyArticle, getArticle } from '@/common';
    import { spawnDisqus } from '@/spawn-disqus';
    import moment from 'moment';

    @Component({
        components: {},
    })
    export default class SingleArticle extends Vue {
        @Prop(String) protected articleId!: string;
        private article: Article = emptyArticle();
        private loaded = false;

        get creationTime() {
            return moment(this.article.date).fromNow();
        }


        protected async beforeMount() {
            try {
                this.article = await getArticle(this.articleId);
                this.loaded = true;
                spawnDisqus(this.articleId);
            } catch (_) {
                this.$router.replace({ name: 'home' });
            }
        }
    }
</script>

<style lang="scss">
  @import "../styles/common";
  @import "../styles/article";

  .SingleArticle {
    display: grid;
    grid-template-columns: 1fr $article-width 1fr;
    grid-template-rows: 3rem 1fr 1fr;
    grid-template-areas:
        "left top right"
        "left body right"
        "left comments right";
  }

  #disqus_thread {
    grid-area: comments;
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
          "body"
          "comments"
    }
  }


</style>
