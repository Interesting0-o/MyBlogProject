<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import AppLink from '@/components/AppLink.vue'
import type { AdminTab } from '@/composables/useRoute'
import { getAdminStats } from '@/services/admin'
import type { StatsSummary } from '@/types/blog'

const props = defineProps<{
  tab: AdminTab
}>()

const emit = defineEmits<{
  logout: []
}>()

interface DockItem {
  tab: AdminTab
  label: string
  short: string
  href: string
  icon: string
}

const items: DockItem[] = [
  {
    tab: 'stats',
    label: '统计',
    short: '统计',
    href: '/admin/stats',
    icon: '<path d="M3 3v18h18"/><path d="m3 17 6-6 4 4 8-8"/><path d="M14 7h7v7"/>',
  },
  {
    tab: 'posts',
    label: '文章管理',
    short: '文章',
    href: '/admin/posts',
    icon: '<path d="M8 6h13M8 12h13M8 18h13"/><path d="M3.5 6h.01M3.5 12h.01M3.5 18h.01"/>',
  },
  {
    tab: 'compose',
    label: '发表文章',
    short: '写作',
    href: '/admin/compose',
    icon: '<path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"/>',
  },
  {
    tab: 'comments',
    label: '评论',
    short: '评论',
    href: '/admin/comments',
    icon: '<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2Z"/>',
  },
  {
    tab: 'settings',
    label: '站点设置',
    short: '设置',
    href: '/admin/settings',
    icon: '<path d="M4 21v-7M4 10V3M12 21v-9M12 8V3M20 21v-5M20 12V3"/><path d="M1 14h6M9 8h6M17 16h6"/>',
  },
]

const logoutIcon =
  '<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><path d="m16 17 5-5-5-5M21 12H9"/>'

// 退出前二次确认
const confirmLogout = ref(false)

function requestLogout() {
  confirmLogout.value = true
}

function cancelLogout() {
  confirmLogout.value = false
}

function handleLogout() {
  confirmLogout.value = false
  emit('logout')
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    cancelLogout()
  }
}

watch(confirmLogout, (open) => {
  if (open) {
    window.addEventListener('keydown', onKeydown)
  } else {
    window.removeEventListener('keydown', onKeydown)
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKeydown)
})

const stats = ref<StatsSummary | null>(null)

async function loadStats() {
  try {
    stats.value = await getAdminStats()
  } catch {
    stats.value = null
  }
}

watch(
  () => props.tab,
  () => {
    void loadStats()
  },
  { immediate: true },
)

const pendingComments = computed(() => stats.value?.pendingComments ?? 0)

const heading = computed(() => {
  const s = stats.value

  switch (props.tab) {
    case 'posts':
      return {
        title: '博客管理',
        description: s
          ? `共 ${s.publishedPosts + s.draftPosts} 篇文章 · ${s.publishedPosts} 篇已发布`
          : '管理已发布与待完善的文章',
      }
    case 'compose':
      return {
        title: '撰写文章',
        description: '支持 Markdown 语法，保存后即时发布',
      }
    case 'drafts':
      return {
        title: '草稿箱',
        description: s
          ? `${s.draftPosts} 篇草稿等待继续完善`
          : '未发布的草稿会保存在这里',
      }
    case 'comments':
      return {
        title: '评论管理',
        description:
          s && s.pendingComments > 0
            ? `有 ${s.pendingComments} 条评论等待审核`
            : '所有评论都已处理完毕',
      }
    case 'settings':
      return {
        title: '站点设置',
        description: '站点信息、主页内容与关于页面',
      }
    case 'stats':
      return {
        title: '数据概览',
        description: s
          ? `总阅读量 ${(s.totalViews ?? 0).toLocaleString()} · 待审核评论 ${s.pendingComments ?? 0}`
          : '博客的阅读与互动数据',
      }
  }
})

function badgeText(count: number) {
  return count > 99 ? '99+' : String(count)
}
</script>

<template>
  <section class="admin-page page-with-glow">
    <!-- 桌面端：左侧竖直 Dock -->
    <nav class="admin-dock admin-dock--desktop" aria-label="后台导航">
      <div class="dock-logo" title="sudo4" aria-hidden="true">S</div>

      <div class="dock-group">
        <AppLink
          v-for="item in items"
          :key="item.tab"
          class="dock-item"
          :class="{ active: tab === item.tab }"
          :href="item.href"
          :aria-label="item.label"
          :aria-current="tab === item.tab ? 'page' : undefined"
        >
          <svg
            viewBox="0 0 24 24"
            width="21"
            height="21"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            v-html="item.icon"
          ></svg>
          <span
            v-if="item.tab === 'comments' && pendingComments"
            class="dock-badge"
            >{{ badgeText(pendingComments) }}</span
          >
          <span class="dock-tip" aria-hidden="true">{{ item.label }}</span>
        </AppLink>
      </div>

      <button
        class="dock-item dock-logout"
        type="button"
        aria-label="退出登录"
        @click="requestLogout"
      >
        <svg
          viewBox="0 0 24 24"
          width="20"
          height="20"
          fill="none"
          stroke="currentColor"
          stroke-width="1.8"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
          v-html="logoutIcon"
        ></svg>
        <span class="dock-tip" aria-hidden="true">退出登录</span>
      </button>
    </nav>

    <!-- 移动端：底部浮动 Dock -->
    <nav class="admin-dock admin-dock--mobile" aria-label="后台导航">
      <div class="dock-pill">
        <AppLink
          v-for="item in items"
          :key="item.tab"
          class="dock-item"
          :class="{ active: tab === item.tab }"
          :href="item.href"
          :aria-label="item.label"
          :aria-current="tab === item.tab ? 'page' : undefined"
        >
          <svg
            viewBox="0 0 24 24"
            width="20"
            height="20"
            fill="none"
            stroke="currentColor"
            stroke-width="1.9"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            v-html="item.icon"
          ></svg>
          <span class="dock-label">{{ item.short }}</span>
          <span
            v-if="item.tab === 'comments' && pendingComments"
            class="dock-dot"
            aria-hidden="true"
          ></span>
        </AppLink>
      </div>
    </nav>

    <div class="admin-main">
      <header class="admin-page-header">
        <div class="page-heading">
          <span class="page-kicker">ADMIN</span>
          <h1>{{ heading.title }}</h1>
          <p>{{ heading.description }}</p>
        </div>

        <button
          class="page-logout"
          type="button"
          aria-label="退出登录"
          @click="requestLogout"
        >
          <svg
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            v-html="logoutIcon"
          ></svg>
        </button>
      </header>

      <div class="admin-content">
        <slot></slot>
      </div>
    </div>

    <!-- 退出确认弹窗 -->
    <Transition name="dialog">
      <div
        v-if="confirmLogout"
        class="dialog-overlay"
        @click.self="cancelLogout"
      >
        <div
          class="dialog-card"
          role="alertdialog"
          aria-modal="true"
          aria-labelledby="logout-dialog-title"
        >
          <h2 id="logout-dialog-title">退出登录？</h2>
          <p>退出后需要重新输入密码才能进入后台。</p>

          <div class="dialog-actions">
            <button class="dialog-button" type="button" @click="cancelLogout">
              取消
            </button>
            <button
              class="dialog-button dialog-button--danger"
              type="button"
              @click="handleLogout"
            >
              退出登录
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </section>
</template>

<style scoped>
.admin-page {
  padding-top: 0;
}

/* ---- 桌面端 Dock ---- */

.admin-dock--desktop {
  position: fixed;
  top: calc(var(--header-height) + 1.25rem);
  bottom: 1.5rem;
  left: 0;
  z-index: 40;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.5rem;
  width: 76px;
  padding-top: 0.25rem;
}

.dock-logo {
  display: grid;
  width: 2.75rem;
  height: 2.75rem;
  place-items: center;
  border-radius: 14px;
  background: var(--accent);
  color: var(--accent-contrast);
  font-family: var(--font-mono);
  font-size: 1.2rem;
  font-weight: 700;
  box-shadow: var(--shadow-sm);
  user-select: none;
}

.dock-group {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  padding: 0.5rem 0.4rem;
  border: 1px solid var(--border);
  border-radius: 22px;
  background: var(--surface);
  box-shadow: var(--shadow-sm);
}

.dock-item {
  position: relative;
  display: grid;
  width: 2.75rem;
  height: 2.75rem;
  place-items: center;
  border-radius: 15px;
  color: var(--text-muted);
  transition:
    color 0.2s ease,
    background-color 0.2s ease;
}

.dock-item:hover {
  color: var(--accent);
  background: var(--surface-soft);
}

.dock-item.active {
  color: var(--accent-contrast);
  background: var(--accent);
}

.dock-logout {
  margin-top: auto;
  border: 0;
  color: var(--danger);
  background: var(--surface);
  cursor: pointer;
}

.dock-logout:hover {
  color: var(--danger);
  background: var(--danger-soft);
}

.dock-badge {
  position: absolute;
  top: 0.25rem;
  right: 0.25rem;
  display: grid;
  min-width: 1rem;
  height: 1rem;
  padding: 0 0.25rem;
  place-items: center;
  border-radius: 999px;
  background: var(--danger);
  color: #ffffff;
  box-shadow: 0 0 0 2px var(--surface);
  font-family: var(--font-mono);
  font-size: 0.625rem;
  font-weight: 700;
  line-height: 1;
}

.dock-tip {
  position: absolute;
  top: 50%;
  left: calc(100% + 0.75rem);
  z-index: 50;
  padding: 0.3rem 0.85rem;
  border-radius: 999px;
  background: var(--text);
  color: var(--bg);
  font-size: var(--text-label);
  font-weight: 500;
  white-space: nowrap;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.15s ease;
  transform: translateY(-50%);
}

.dock-item:hover .dock-tip,
.dock-item:focus-visible .dock-tip {
  opacity: 1;
}

/* ---- 移动端 Dock ---- */

.admin-dock--mobile {
  display: none;
}

/* ---- 主内容区 ---- */

.admin-main {
  width: min(1120px, 100%);
  margin-inline: auto;
  padding: clamp(2rem, 5vw, 3.25rem) clamp(1.25rem, 4vw, 2.5rem) 3.5rem;
}

.admin-page-header {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  justify-content: space-between;
  gap: 1rem;
}

.page-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.page-heading h1 {
  margin-top: 0.4rem;
  font-size: clamp(1.7rem, 3.4vw, 2.1rem);
  letter-spacing: -0.04em;
}

.page-heading p {
  margin-top: 0.35rem;
  color: var(--text-muted);
  font-size: var(--text-meta);
}

/* 移动端专属：固定在右上角，不随内容滚动；桌面端退出入口在左侧 Dock 底部 */
.page-logout {
  position: fixed;
  top: calc(var(--header-height) + 0.875rem);
  right: 0.875rem;
  z-index: 45;
  display: none;
  width: 2.75rem;
  height: 2.75rem;
  place-items: center;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--danger);
  background: var(--header-bg);
  backdrop-filter: blur(18px);
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.admin-content {
  display: grid;
  gap: 1.25rem;
  align-items: start;
  margin-top: 1.75rem;
}

/* 允许面板在 grid 中收缩，避免内部宽表格把内容区撑出视口 */
.admin-content > * {
  min-width: 0;
}

/* ---- 退出确认弹窗 ---- */

.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 60;
  display: grid;
  place-items: center;
  padding: 1.5rem;
  background: rgba(8, 14, 12, 0.45);
  backdrop-filter: blur(6px);
}

.dialog-card {
  width: min(23rem, 100%);
  padding: 1.5rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  background: var(--surface);
  box-shadow: var(--shadow-lg);
}

.dialog-card h2 {
  font-size: 1.15rem;
  letter-spacing: -0.02em;
}

.dialog-card p {
  margin-top: 0.5rem;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.7;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.6rem;
  margin-top: 1.5rem;
}

.dialog-button {
  display: inline-flex;
  min-height: 2.5rem;
  align-items: center;
  justify-content: center;
  padding: 0 1.1rem;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-soft);
  background: var(--surface);
  font-size: var(--text-small);
  font-weight: 500;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.dialog-button:hover {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.dialog-button--danger {
  border-color: transparent;
  color: #ffffff;
  background: var(--danger);
}

.dialog-button--danger:hover {
  border-color: transparent;
  color: #ffffff;
  background: color-mix(in srgb, var(--danger) 85%, black);
}

.dialog-enter-active,
.dialog-leave-active {
  transition: opacity 0.18s ease;
}

.dialog-enter-active .dialog-card,
.dialog-leave-active .dialog-card {
  transition: transform 0.18s ease;
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-from .dialog-card,
.dialog-leave-to .dialog-card {
  transform: scale(0.96) translateY(6px);
}

/*
 * Dock 栏转移断点：内容区最宽 1120px，dock 固定宽 76px。
 * 内容左边距 = (100vw - 1120px) / 2，当它小于 dock 宽度（100vw < 1120px + 76px * 2）
 * 时 dock 会挡住主要内容，此时转移到底部。
 */
@media (max-width: 1272px) {
  .admin-page {
    padding-bottom: calc(6.75rem + env(safe-area-inset-bottom));
  }

  .admin-dock--desktop {
    display: none;
  }

  .admin-dock--mobile {
    position: fixed;
    inset-inline: 0;
    bottom: 0;
    z-index: 40;
    display: flex;
    justify-content: center;
    padding: 0 1rem calc(env(safe-area-inset-bottom) + 0.875rem);
    pointer-events: none;
  }

  .dock-pill {
    display: flex;
    align-items: stretch;
    gap: 0.15rem;
    width: min(28rem, 100%);
    padding: 0.4rem;
    border: 1px solid var(--border);
    border-radius: 24px;
    background: var(--header-bg);
    backdrop-filter: blur(18px);
    box-shadow: var(--shadow-lg);
    pointer-events: auto;
  }

  .dock-pill .dock-item {
    flex: 1;
    min-width: 0;
    height: 3.1rem;
    border-radius: 16px;
  }

  .dock-label {
    font-size: 0.65rem;
    font-weight: 500;
    line-height: 1;
  }

  .dock-dot {
    position: absolute;
    top: 0.5rem;
    right: 30%;
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 999px;
    background: var(--danger);
    box-shadow: 0 0 0 2px var(--surface);
  }

  .page-logout {
    display: grid;
    place-items: center;
  }

  .page-logout:hover {
    border-color: var(--danger-line);
    background: var(--danger-soft);
  }

  .admin-main {
    padding-top: 1.75rem;
    padding-bottom: 0;
  }

  .admin-content {
    margin-top: 1.25rem;
  }
}
</style>
