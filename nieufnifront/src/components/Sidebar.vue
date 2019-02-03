<template>
  <div class="Sidebar">
    this is <strong>Sidebar</strong>
  </div>
  
</template>

<script lang="ts">
    import {Component, Vue} from 'vue-property-decorator';

    @Component({
        components: {},
    })
    export default class Sidebar extends Vue {
    }
</script>

<style scoped lang="scss">
  @import "../styles/navbar";

  .Sidebar {
    grid-area: sidebar;
    border: 2px solid black;
    display: grid;
    align-content: center;
    justify-content: center;
    height: $navbar-height;
    width: 100%;
  }
</style>
