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
  .Sidebar {
    grid-area: sidebar;
    border: 2px solid black;
    display: grid;
    align-content: center;
    justify-content: center;
    height: 100%;
    width: 100%;
  }
</style>
