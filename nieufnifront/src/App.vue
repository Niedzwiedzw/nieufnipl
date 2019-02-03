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
    export default class App extends Vue {}
</script>

<style lang="scss">
  @import './styles/common';
  @import './styles/colors';
  @import './styles/navbar.scss';

  #app {
    width: 100%;
    height: 100%;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: $navbar-height 1fr
  }

  #page {
    padding: 2rem;
  }

  body, html {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    font-family: 'Playfair Display', serif;

    background-color: $primary-white;
  }

  @media only screen and (max-width: $mobile-threshold) {
    #page {
      padding: 0.8rem;
    }
  }



  *::-moz-focus-inner {
    border: 0;
  }
  *:focus {outline:none;}

</style>
