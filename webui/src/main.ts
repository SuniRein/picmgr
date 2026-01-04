import { injectRouter } from './api';
import App from './App.vue';
import router from './router';
import './style.css';

const pinia = createPinia();

injectRouter(router);

createApp(App)
  .use(router)
  .use(pinia)
  .mount('#app');
