import { createApp } from 'vue'
import { createPinia } from 'pinia'
import './style.css'
import App from './App.vue'
import router from './router'
import { setupGlobalErrorHandlers, log } from './utils/log'

// 创建应用
const app = createApp(App)

// 安装插件
const pinia = createPinia()
app.use(pinia)
app.use(router)

// 设置全局错误处理
setupGlobalErrorHandlers()

// 挂载应用
app.mount('#app')

// 记录启动日志
log.info('system', '应用启动成功')
