/**
 * Vue Router 配置
 */

import { createRouter, createWebHashHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Dashboard',
    component: () => import('@/views/Dashboard.vue'),
    meta: { title: '首页' },
  },
  {
    path: '/backtest/config',
    name: 'BacktestConfig',
    component: () => import('@/views/BacktestConfig.vue'),
    meta: { title: '回测配置' },
  },
  {
    path: '/backtest/result/:id',
    name: 'BacktestResult',
    component: () => import('@/views/BacktestResult.vue'),
    meta: { title: '回测结果' },
  },
  {
    path: '/data',
    name: 'DataManager',
    component: () => import('@/views/DataManager.vue'),
    meta: { title: '数据管理' },
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
