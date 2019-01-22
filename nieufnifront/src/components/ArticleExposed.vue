<template>
  <div class="ArticleExposed">
    this is <strong>ArticleExposed</strong>
  </div>
  
</template>

<script lang="ts">
    import {Component, Vue} from 'vue-property-decorator';

    @Component({
        components: {},
    })
    export default class ArticleExposed extends Vue {}
</script>

<style scoped lang="scss">
  .ArticleExposed {

  }
</style>
