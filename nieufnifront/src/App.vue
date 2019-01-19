<template>
  <div id="app">
    <Navbar/>
    <div id="page">
      <router-view/>
    </div>
  </div>
</template>

<script lang="ts">
    import { Component, Vue } from 'vue-property-decorator';

    import Navbar from './components/Navbar.vue';

    @Component({
        components: {
            Navbar,
        },
    })
    export default class App extends Vue {
    }
</script>

<style lang="scss">
  @import './styles/common';
  #app {
    width: 100%;
    height: 100%;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: 5rem 1fr
  }

  .navbar-link {

  }

  #page {

  }

  body, html {
    width: 100%;
    height: 100%;
    overflow: hidden;
    margin: 0;
  }
</style>
