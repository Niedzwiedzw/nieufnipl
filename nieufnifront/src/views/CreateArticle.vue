<template>
  <div class="CreateArticle">
    <div class="autor">
      <input v-model="articleMeta.username" type="text" class="credentials" placeholder="nazwa użytkownika">
      <input v-model="articleMeta.password" type="text" class="credentials" placeholder="hasło">
      <input v-model="articleMeta.articleTitle" type="text" class="credentials" placeholder="tytul artykulu">
      <button
          @click="sendArticle"
          v-show="articleMeta.username && articleMeta.password && articleMeta.articleTitle"
      >
        wyślij
      </button>
      <div style="color: blueviolet">{{ JSON.stringify(articleMeta) }}</div>
      <p v-if="errors">Coś poszło nietak, sprawdź nazwę uśytkownika, hasło i tytuł.</p>
    </div>
    <div class="article-body rendered" v-html="renderedText"></div>
    <div class="input">
      <textarea v-model="textInput"></textarea>
    </div>

  </div>

</template>

<script lang="ts">
    import showdown from 'showdown';
    import { Component, Vue } from 'vue-property-decorator';
    import { NewArticleData, sendArticle } from '@/common';

    @Component({
        components: {},
    })
    export default class CreateArticle extends Vue {
        private articleMeta: NewArticleData = { username: '', password: '', articleTitle: ''};
        private errors: boolean = false;
        private textInput: string = '# Nowy artykul\n' +
            '\n' +
            'edytuj se to\n' +
            '\n' +
            '![masujemy]' +
            '(https://scontent-frt3-2.xx.fbcdn.net/v/t1.0-9/30709455_10204563936054577_2661861064651571200_n.jpg' +
            '?_nc_cat=110&_nc_ht=scontent-frt3-2.xx&oh=144a56a964b27ff7bb4a7f7ac5885eed&oe=5CFFFD10)\n' +
            'Format: ![Alt Text](url)\n';
        private converter = new showdown.Converter();
        public get renderedText(): string { return this.converter.makeHtml(this.textInput); }

        public async sendArticle() {
            const response = await sendArticle(
                this.articleMeta.articleTitle,
                this.textInput,
                this.articleMeta.username,
                this.articleMeta.password,
            );

            this.errors = await await response;
        }
    }
</script>

<style lang="scss">
  @import "../styles/common";
  @import "../styles/article";

  .CreateArticle {
    display: grid;
    grid-template-columns: 1fr $article-width 1fr;
    grid-template-rows: 5rem 1.3fr 1fr;

    button {
      height: 2rem;
      background: lightgray;
      color: hotpink;
    }
  }

  .autor {
    grid-column: 2;
  }
  .input {
    grid-column: 2 ;
    grid-row: 3;

    textarea {
      width: 100%;
      height: 100%;
    }
  }

  .rendered {
    grid-column: 2;
    grid-row: 2;
  }
</style>
