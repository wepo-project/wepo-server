import { createApp } from 'vue'
import App from './App.vue'
import Router from './pageRouter';
import store from './store';
import "./index.css"

const app = createApp(App)

app.use(Router)
app.use(store)

app.mount('#app')
