<script setup lang="ts">
import { onMounted, ref } from 'vue'
import AdminCommentsPanel from '@/components/admin/AdminCommentsPanel.vue'
import AdminComposePanel from '@/components/admin/AdminComposePanel.vue'
import AdminLogin from '@/components/admin/AdminLogin.vue'
import AdminPostsPanel from '@/components/admin/AdminPostsPanel.vue'
import AdminSettingsPanel from '@/components/admin/AdminSettingsPanel.vue'
import AdminShell from '@/components/admin/AdminShell.vue'
import AdminStatsPanel from '@/components/admin/AdminStatsPanel.vue'
import type { AdminTab } from '@/composables/useRoute'
import { checkSession, logout } from '@/services/auth'

const props = defineProps<{
  tab: AdminTab
}>()

const authState = ref<'checking' | 'login' | 'ready'>('checking')

onMounted(async () => {
  authState.value = (await checkSession()) ? 'ready' : 'login'
})

function handleLoginSuccess() {
  authState.value = 'ready'
}

async function handleLogout() {
  await logout()
  authState.value = 'login'
}
</script>

<template>
  <AdminLogin v-if="authState === 'login'" @success="handleLoginSuccess" />

  <section v-else-if="authState === 'checking'" class="admin-page page-with-glow container">
    <p class="checking">检查登录状态…</p>
  </section>

  <AdminShell v-else :tab="props.tab" @logout="handleLogout">
    <AdminPostsPanel
      v-if="props.tab === 'posts' || props.tab === 'drafts'"
      :initial-filter="props.tab === 'drafts' ? 'draft' : 'all'"
    />
    <AdminComposePanel v-else-if="props.tab === 'compose'" />
    <AdminCommentsPanel v-else-if="props.tab === 'comments'" />
    <AdminStatsPanel v-else-if="props.tab === 'stats'" />
    <AdminSettingsPanel v-else />
  </AdminShell>
</template>

<style scoped>
.checking {
  padding: 4rem 0;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-small);
}


</style>
