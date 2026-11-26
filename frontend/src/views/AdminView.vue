<script setup lang="ts">
import { onMounted, ref } from 'vue'
import AdminCommentsPanel from '@/components/admin/AdminCommentsPanel.vue'
import AdminComposePanel from '@/components/admin/AdminComposePanel.vue'
import AdminDraftsPanel from '@/components/admin/AdminDraftsPanel.vue'
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

  <AdminShell v-else :tab="props.tab">
    <template #nav-footer>
      <button class="logout-button" type="button" @click="handleLogout">退出登录</button>
    </template>

    <AdminPostsPanel v-if="props.tab === 'posts'" />
    <AdminComposePanel v-else-if="props.tab === 'compose'" />
    <AdminDraftsPanel v-else-if="props.tab === 'drafts'" />
    <AdminCommentsPanel v-else-if="props.tab === 'comments'" />
    <AdminSettingsPanel v-else-if="props.tab === 'settings'" />
    <AdminStatsPanel v-else />
  </AdminShell>
</template>

<style scoped>
.checking {
  padding: 4rem 0;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-small);
}

.logout-button {
  padding: 0.55rem 0.75rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  color: var(--danger);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  transition:
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.logout-button:hover {
  border-color: var(--danger-line);
  background: var(--danger-soft);
}
</style>
