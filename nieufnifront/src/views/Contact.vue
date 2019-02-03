<template>
  <div class="contact">
    <div class="container">
      <h1>Kontakt</h1>
      <p>mail: niedzwiedzwo@gmail.com</p>
    </div>
  </div>
</template>

<script lang="ts">
    import { Component, Vue } from 'vue-property-decorator';

    @Component({
        components: {
        },
    })
    export default class Home extends Vue {}
</script>
<style lang="scss">
  @import '../styles/common';

  .contact {
    display:flex;
    justify-content:center;
    align-items:center;
  }
</style>
