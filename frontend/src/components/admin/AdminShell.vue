<script setup lang="ts">
import AppLink from '@/components/AppLink.vue'
import type { AdminTab } from '@/composables/useRoute'

defineProps<{
  tab: AdminTab
}>()

const tabs: { tab: AdminTab; label: string; href: string }[] = [
  { tab: 'posts', label: '文章管理', href: '/admin/posts' },
  { tab: 'compose', label: '发表文章', href: '/admin/compose' },
  { tab: 'drafts', label: '草稿管理', href: '/admin/drafts' },
  { tab: 'comments', label: '评论', href: '/admin/comments' },
  { tab: 'settings', label: '站点设置', href: '/admin/settings' },
  { tab: 'stats', label: '统计', href: '/admin/stats' },
]
</script>

<template>
  <section class="admin-page page-with-glow container">
    <header class="admin-header">
      <span class="page-kicker">ADMIN PANEL</span>
      <h1>后台管理</h1>
    </header>

    <div class="admin-shell">
      <aside class="admin-nav" aria-label="后台导航">
        <nav>
          <AppLink
            v-for="item in tabs"
            :key="item.tab"
            :href="item.href"
            :class="{ active: tab === item.tab }"
            :aria-current="tab === item.tab ? 'page' : undefined"
          >
            <span>{{ item.label }}</span>
          </AppLink>
        </nav>
        <slot name="nav-footer"></slot>
      </aside>

      <div class="admin-content">
        <slot></slot>
      </div>
    </div>
  </section>
</template>

<style scoped>
.admin-page {
  padding-top: clamp(2.5rem, 5vw, 4rem);
}

.page-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.admin-header h1 {
  margin-top: 0.5rem;
  font-size: clamp(1.9rem, 4vw, 2.6rem);
  letter-spacing: -0.045em;
}

.admin-shell {
  display: grid;
  margin-top: 1.75rem;
  grid-template-columns: 11rem minmax(0, 1fr);
  align-items: start;
  gap: clamp(1.25rem, 3vw, 2rem);
}

.admin-nav {
  position: sticky;
  top: calc(var(--header-height) + 1.5rem);
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.admin-nav nav {
  display: grid;
  gap: 2px;
  padding: 0.4rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.admin-nav a {
  padding: 0.55rem 0.75rem;
  border-radius: var(--radius-sm);
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  transition:
    color 0.2s ease,
    background-color 0.2s ease;
}

.admin-nav a:hover {
  color: var(--text);
  background: var(--surface-soft);
}

.admin-nav a.active {
  color: var(--accent);
  background: var(--accent-soft);
}

@media (max-width: 860px) {
  .admin-shell {
    grid-template-columns: 1fr;
  }

  .admin-nav {
    position: static;
  }

  .admin-nav nav {
    display: flex;
    overflow-x: auto;
  }

  .admin-nav a {
    white-space: nowrap;
  }
}
</style>
