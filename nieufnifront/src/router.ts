import Vue from 'vue';
import Router from 'vue-router';
import Home from './views/Home.vue';

Vue.use(Router);

export default new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/artykuly',
      name: 'articles',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "artykuly" */ './views/Articles.vue'),
    },
    {
      path: '/kontakt',
      name: 'contact',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "kontakt" */ './views/Contact.vue'),
    },
    {
      path: '/redakcja',
      name: 'people',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "redakcja" */ './views/People.vue'),
    },
    {
      path: '/:articleId',
      name: 'singlearticle',
      props: true,
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "artykul" */ './views/SingleArticle.vue'),
    },
    {
      path: '/redakcja/nowy-artykul',
      name: 'articlecreate',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import(/* webpackChunkName: "nowyartykul" */ './views/CreateArticle.vue'),
    },
  ],
});
