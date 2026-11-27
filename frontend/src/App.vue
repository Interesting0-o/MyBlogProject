<script setup lang="ts">
import { onBeforeUnmount, onMounted, watch } from 'vue'
import AppFooter from '@/components/AppFooter.vue'
import AppHeader from '@/components/AppHeader.vue'
import { useRoute, type Route } from '@/composables/useRoute'
import { useTheme } from '@/composables/useTheme'
import AboutView from '@/views/AboutView.vue'
import AdminView from '@/views/AdminView.vue'
import ArchiveView from '@/views/ArchiveView.vue'
import HomeView from '@/views/HomeView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import PostView from '@/views/PostView.vue'
import { isTypingTarget } from '@/utils/dom'

const { route } = useRoute()
const { toggleTheme } = useTheme()

const documentTitles: Record<Route['name'], string> = {
  home: 'sudo4 · 在终端之外记录',
  post: '文章 · sudo4',
  archive: '归档 · sudo4',
  about: '关于 · sudo4',
  admin: '后台 · sudo4',
  'not-found': '404 · sudo4',
}

function handleShortcut(event: KeyboardEvent) {
  if (
    event.defaultPrevented ||
    event.repeat ||
    event.metaKey ||
    event.ctrlKey ||
    event.altKey ||
    event.key.toLowerCase() !== 't' ||
    isTypingTarget(event.target)
  ) {
    return
  }

  toggleTheme()
}

watch(
  route,
  (current) => {
    document.title = documentTitles[current.name]
    window.scrollTo(0, 0)
  },
  { immediate: true },
)

onMounted(() => {
  window.addEventListener('keydown', handleShortcut)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleShortcut)
})
</script>

<template>
  <div class="site-shell">
    <AppHeader />

    <main id="main-content" class="site-main">
      <Transition name="view" mode="out-in">
        <HomeView v-if="route.name === 'home'" key="home" />
        <PostView
          v-else-if="route.name === 'post'"
          :key="`post-${route.slug}`"
          :slug="route.slug"
        />
        <ArchiveView v-else-if="route.name === 'archive'" key="archive" />
        <AboutView v-else-if="route.name === 'about'" key="about" />
        <AdminView v-else-if="route.name === 'admin'" key="admin" :tab="route.tab" />
        <NotFoundView v-else key="not-found" />
      </Transition>
    </main>

    <AppFooter />
  </div>
</template>
