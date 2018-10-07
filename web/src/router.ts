import Vue from 'vue';
import Router from 'vue-router';
import Start from '@/views/Start.vue';
import Login from '@/views/Login.vue';
import Signup from '@/views/Signup.vue';
import Home from '@/views/Home.vue';
import Task from '@/views/Task.vue';
import Articles from '@/views/Articles.vue';
import NewArticle from '@/views/NewArticle.vue';
import Article from '@/views/Article.vue';
import EditArticle from '@/views/EditArticle.vue';

Vue.use(Router);

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [{
    path: '/',
    name: 'start',
    component: Start,
  }, {
    path: '/login',
    name: 'login',
    component: Login,
  }, {
    path: '/signup',
    name: 'signup',
    component: Signup,
  }, {
    path: '/home',
    name: 'home',
    component: Home,
  }, {
    path: '/task',
    name: 'task',
    component: Task,
  }, {
    path: '/article',
    name: 'list-article',
    component: Articles,
  }, {
    path: '/article/new',
    name: 'new-article',
    component: NewArticle,
  }, {
    path: '/article/:title',
    name: 'show-article',
    component: Article,
  }, {
    path: '/article/:title/edit',
    name: 'edit-article',
    component: EditArticle,
  }],
});
