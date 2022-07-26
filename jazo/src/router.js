import Vue from 'vue'
import Router from 'vue-router'
import PoView from './views/PoView.vue'
import HomeView from './views/HomeView.vue'
import MomentView from './views/MomentView.vue'
import ExperiencesView from './views/ExperiencesView.vue'
import AboutView from './views/AboutView.vue'
import StatsView from './views/StatsView.vue'

Vue.use(Router)

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },    
    {
      path: '/moment/:video_id?',
      name: 'moment',
      component: MomentView
    },
    {
      path: '/experiences',
      name: 'experiencesview',
      component: ExperiencesView
    },
    {
      path: '/pov/:video_id?',
      name: 'pov',
      component: PoView
    },
    {
      path: '/stats',
      name: 'stats',
      component: StatsView
    },
    {
      path: '/about',
      name: 'about',
      component: AboutView
    }
  ]
})
