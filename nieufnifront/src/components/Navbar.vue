<template>
  <div class="Navbar">
    <router-link class="hoverable navbar-link home-link" to="/">nieufni.pl</router-link>
    <router-link class="hoverable navbar-link" to="/artykuly">artykuły</router-link>
    <router-link class="hoverable navbar-link" to="/redakcja">redakcja</router-link>
    <router-link class="hoverable navbar-link" to="/kontakt">kontakt</router-link>
  </div>

</template>

<script lang="ts">
    import {Component, Vue} from 'vue-property-decorator';

    @Component({
        components: {},
    })
    export default class Navbar extends Vue {
    }
</script>

<style scoped lang="scss">
  .navbar-link {
    width: 100%;
    height: 100%;
    display: grid;
    align-content: center;
    justify-content: center;

    text-decoration: none;
    color: inherit;
  }

  .home-link {

  }

  .Navbar {
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 1fr;
    /*grid-gap: 2rem;*/
    /*padding: 0 2rem;*/

    justify-items: start;
    align-items: center;
    border-bottom: 3px solid black;
  }
</style>
