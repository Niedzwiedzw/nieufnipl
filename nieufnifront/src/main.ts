import Vue from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';
import './registerServiceWorker';
import {setDefaultMeta} from '@/config';

Vue.config.productionTip = false;

setDefaultMeta();  // for facebook scrapers


new Vue({
  router,
  store,
  render: (h) => h(App),
}).$mount('#app');
