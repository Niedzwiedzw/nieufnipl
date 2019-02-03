<template>
  <div class="nav-container">
    <div class="Navbar">
      <router-link class="hoverable navbar-link home-link" to="/">
        <nobr>
          nieufn<nobr class="colored">i</nobr>.pl
        </nobr>

      </router-link>
      <router-link class="hoverable navbar-link" to="/kontakt">kontakt</router-link>
    </div>
  </div>

</template>

<script lang="ts">
    import {Component, Vue} from 'vue-property-decorator';

    @Component({
        components: {},
    })
    export default class Navbar extends Vue {}
</script>

<style scoped lang="scss">
  @import '../styles/navbar';
  @import '../styles/common';

  .nav-container {
    width: 100%;
    height: $navbar-height;
    display: grid;
    grid-template-columns: 1fr $site-width 1fr;
  }

  .navbar-link {
    width: 100%;
    height: 100%;
    display: grid;
    align-content: center;
    justify-content: center;

    text-decoration: none;
    color: inherit;
    font-size: 1.8rem;

    text-shadow: rgba(110, 15, 0, 0.07) 3px 4px 2px;
  }

  .colored {
    color: hotpink;
  }

  .home-link {

  }

  .Navbar {
    display: grid;
    grid-column: 2;
    grid-auto-flow: column;
    grid-auto-columns: 1fr;
    /*grid-gap: 2rem;*/
    /*padding: 0 2rem;*/

    background-color: #ffd7bc;
    color: black;

    justify-items: start;
    align-items: center;
    border-bottom-left-radius: 1.5rem;

    box-shadow: rgba(255, 105, 180, 0.48) 0.1rem 0.1rem 0.4rem 0.01rem;
  }

  @media only screen and (max-width: $site-width) {
    .nav-container {
      grid-template-columns: 1fr;
    }

    .Navbar {
      grid-column: 1;
    }
  }
</style>
