import { createApp } from 'vue'
import { createPinia } from 'pinia'
import './style.css'
import { useAppStore } from './stores/app'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(router).use(createPinia())

app.mount('#app')
useAppStore().initializeConnection()
