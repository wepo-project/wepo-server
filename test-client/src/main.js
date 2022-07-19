import { createApp } from 'vue'
import App from './App.vue'
import Router from './pageRouter';
import store from './store';
import VConsole from "vconsole";
import { initPlugin } from "vue-vconsole-devtools";
import "./index.css"

initPlugin(new VConsole())

const app = createApp(App)


app.use(Router)
app.use(store)

app.mount('#app')
