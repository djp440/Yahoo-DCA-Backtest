<template>
  <div class="app flex flex-col h-full">
    <!-- 顶部导航栏 -->
    <TopNav @menu-change="handleMenuChange" />

    <!-- Tab标签栏 -->
    <TabBar />

    <!-- 工作区 -->
    <Workspace>
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </Workspace>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import TopNav from '@/components/layout/TopNav.vue'
import TabBar from '@/components/layout/TabBar.vue'
import Workspace from '@/components/layout/Workspace.vue'
import { log } from './utils/log'

onMounted(() => {
  log.info('ui', 'App组件挂载完成')
})

function handleMenuChange(key: string) {
  log.info('ui', `菜单切换: ${key}`)
}
</script>

<style scoped>
.app {
  width: 100%;
  height: 100%;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
