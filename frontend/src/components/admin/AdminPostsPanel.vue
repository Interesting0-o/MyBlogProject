<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { navigate } from '@/composables/useRoute'
import {
  deleteAdminPost,
  listAdminPosts,
  updateAdminPost,
  type AdminPost,
} from '@/services/admin'
import { setEditingPost } from '@/stores/adminPost'
import type { PostStatus } from '@/types/blog'

const props = defineProps<{
  initialFilter?: PostStatus | 'all'
}>()

type Filter = PostStatus | 'all'

const posts = ref<AdminPost[]>([])
const filter = ref<Filter>(props.initialFilter ?? 'all')
const loading = ref(false)
const processing = ref<string | null>(null)
const error = ref<string | null>(null)
const message = ref<{ type: 'ok' | 'error'; text: string } | null>(null)

const counts = computed(() => ({
  all: posts.value.length,
  published: posts.value.filter((post) => post.status === 'published').length,
  draft: posts.value.filter((post) => post.status === 'draft').length,
}))

const filters = computed(() => [
  { value: 'all' as Filter, label: '全部', count: counts.value.all },
  { value: 'published' as Filter, label: '已发布', count: counts.value.published },
  { value: 'draft' as Filter, label: '草稿', count: counts.value.draft },
])

const filteredPosts = computed(() =>
  [...posts.value]
    .filter((post) => filter.value === 'all' || post.status === filter.value)
    .sort((a, b) =>
      (b.updatedAt ?? b.publishedAt).localeCompare(a.updatedAt ?? a.publishedAt),
    ),
)

const editIcon =
  '<path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"/>'
const globeIcon =
  '<circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/>'
const undoIcon =
  '<path d="M9 14 4 9l5-5"/><path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5 5.5 5.5 0 0 1-5.5 5.5H11"/>'
const trashIcon =
  '<path d="M3 6h18"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><path d="M10 11v6M14 11v6"/>'

function updatedLabel(post: AdminPost): string {
  return post.updatedAt ?? post.publishedAt
}

async function load() {
  loading.value = true
  error.value = null

  try {
    posts.value = await listAdminPosts()
  } catch (loadError) {
    error.value = loadError instanceof Error ? loadError.message : '加载文章列表失败'
  } finally {
    loading.value = false
  }
}

function edit(post: AdminPost) {
  setEditingPost(post)
  navigate('/admin/compose')
}

function today(): string {
  return new Date().toISOString().slice(0, 10)
}

async function toggleStatus(post: AdminPost) {
  const next: PostStatus = post.status === 'published' ? 'draft' : 'published'
  processing.value = post.slug
  message.value = null

  try {
    await updateAdminPost(post.slug, {
      slug: post.slug,
      title: post.title,
      description: post.description,
      publishedAt: post.publishedAt,
      updatedAt: today(),
      tags: post.tags,
      readingMinutes: post.readingMinutes,
      featured: Boolean(post.featured),
      status: next,
      content: post.content,
    })
    message.value = {
      type: 'ok',
      text: next === 'published' ? `已发布「${post.title}」` : `已将「${post.title}」下线为草稿`,
    }
    await load()
  } catch (toggleError) {
    message.value = {
      type: 'error',
      text: toggleError instanceof Error ? toggleError.message : '操作失败',
    }
  } finally {
    processing.value = null
  }
}

async function remove(post: AdminPost) {
  if (!window.confirm(`删除文章「${post.title}」？此操作不可恢复。`)) {
    return
  }

  processing.value = post.slug
  message.value = null

  try {
    await deleteAdminPost(post.slug)
    message.value = { type: 'ok', text: `已删除「${post.title}」` }
    await load()
  } catch (deleteError) {
    message.value = {
      type: 'error',
      text: deleteError instanceof Error ? deleteError.message : '删除失败',
    }
  } finally {
    processing.value = null
  }
}

onMounted(load)
</script>

<template>
  <section class="admin-panel posts-panel">
    <!-- 筛选 -->
    <div class="posts-toolbar">
      <div class="filter-tabs">
        <button
          v-for="item in filters"
          :key="item.value"
          class="filter-tab"
          :class="{ active: filter === item.value }"
          type="button"
          @click="filter = item.value"
        >
          {{ item.label }}
          <span class="filter-count">{{ item.count }}</span>
        </button>
      </div>

      <button
        class="icon-button"
        type="button"
        title="刷新"
        aria-label="刷新列表"
        :disabled="loading"
        @click="load"
      >
        <svg
          viewBox="0 0 24 24"
          width="15"
          height="15"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="M3 12a9 9 0 0 1 15.3-6.4L21 8" />
          <path d="M21 3v5h-5" />
          <path d="M21 12a9 9 0 0 1-15.3 6.4L3 16" />
          <path d="M3 21v-5h5" />
        </svg>
      </button>
    </div>

    <p v-if="message" class="form-message" :class="`form-message--${message.type}`">
      {{ message.text }}
    </p>

    <p v-if="error" class="panel-state panel-state--error">{{ error }}</p>
    <p v-else-if="loading && !posts.length" class="panel-state">加载中…</p>
    <p v-else-if="!filteredPosts.length" class="panel-state">
      {{ filter === 'all' ? '还没有文章，写第一篇吧。' : '没有符合条件的文章。' }}
    </p>

    <ul v-else class="posts-list">
      <li v-for="post in filteredPosts" :key="post.slug" class="post-card">
        <div class="post-info">
          <div class="post-top">
            <span class="status-badge" :class="`status-badge--${post.status}`">
              <i class="status-dot" aria-hidden="true"></i>
              {{ post.status === 'published' ? '已发布' : '草稿' }}
            </span>
            <span class="post-time">更新于 {{ updatedLabel(post) }}</span>
          </div>

          <strong class="post-title" :title="post.title" @click="edit(post)">
            {{ post.title }}
          </strong>

          <p v-if="post.description" class="post-desc">{{ post.description }}</p>

          <div class="post-meta">
            <span v-if="post.tags.length" class="post-tags">
              <span v-for="tag in post.tags" :key="tag" class="tag-chip">{{ tag }}</span>
            </span>
            <span class="post-meta-item">/posts/{{ post.slug }}</span>
            <span class="post-meta-item">{{ post.views ?? 0 }} 次阅读</span>
          </div>
        </div>

        <div class="row-actions">
          <button
            class="round-action"
            type="button"
            title="编辑"
            :aria-label="`编辑 ${post.title}`"
            @click="edit(post)"
          >
            <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" v-html="editIcon"></svg>
          </button>
          <button
            class="round-action round-action--accent"
            type="button"
            :title="post.status === 'published' ? '下线为草稿' : '直接发布'"
            :aria-label="post.status === 'published' ? `下线 ${post.title}` : `发布 ${post.title}`"
            :disabled="processing === post.slug"
            @click="toggleStatus(post)"
          >
            <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" v-html="post.status === 'published' ? undoIcon : globeIcon"></svg>
          </button>
          <button
            class="round-action round-action--danger"
            type="button"
            title="删除"
            :aria-label="`删除 ${post.title}`"
            :disabled="processing === post.slug"
            @click="remove(post)"
          >
            <svg viewBox="0 0 24 24" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" v-html="trashIcon"></svg>
          </button>
        </div>
      </li>
    </ul>
  </section>
</template>

<style scoped>
.admin-panel {
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  background: var(--surface);
}

/* ---- 筛选工具条 ---- */

.posts-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border);
}

.filter-tabs {
  display: flex;
  gap: 0.25rem;
  padding: 0.25rem;
  border-radius: 999px;
  background: var(--surface-soft);
}

.filter-tab {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  height: 2.1rem;
  padding: 0 0.9rem;
  border-radius: 999px;
  color: var(--text-muted);
  background: transparent;
  font-size: var(--text-label);
  font-weight: 500;
  white-space: nowrap;
  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    box-shadow 0.2s ease;
}

.filter-tab.active {
  color: var(--text);
  background: var(--surface);
  box-shadow: var(--shadow-sm);
}

.filter-count {
  padding: 0 0.4rem;
  border-radius: 999px;
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: 0.7rem;
  line-height: 1.3;
}

.filter-tab.active .filter-count {
  color: var(--accent-contrast);
  background: var(--accent);
}

.icon-button {
  display: grid;
  width: 2.1rem;
  height: 2.1rem;
  place-items: center;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-muted);
  background: var(--surface);
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.icon-button:hover:not(:disabled) {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.icon-button:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

/* ---- 状态与提示 ---- */

.panel-state {
  margin: 0;
  padding: 2.5rem 1.25rem;
  color: var(--text-muted);
  font-size: var(--text-small);
  text-align: center;
}

.panel-state--error {
  color: var(--danger);
}

.form-message {
  margin: 0.85rem 1rem 0;
  padding: 0.6rem 0.8rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  font-size: var(--text-small);
}

.form-message--ok {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.form-message--error {
  border-color: var(--danger-line);
  color: var(--danger);
  background: var(--danger-soft);
}

/* ---- 文章卡片 ---- */

.posts-list {
  margin: 0;
  padding: 0.5rem;
  list-style: none;
}

.post-card {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  padding: 1rem 1.1rem;
  border-radius: var(--radius-md);
  transition: background-color 0.2s ease;
}

.post-card:hover {
  background: var(--surface-soft);
}

.post-info {
  display: flex;
  min-width: 0;
  flex: 1;
  flex-direction: column;
  gap: 0.45rem;
}

.post-top {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.6rem;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.18rem 0.6rem;
  border-radius: 999px;
  font-size: var(--text-label);
  font-weight: 500;
}

.status-dot {
  width: 0.375rem;
  height: 0.375rem;
  border-radius: 999px;
  background: currentColor;
}

.status-badge--published {
  color: var(--accent);
  background: var(--accent-soft);
}

.status-badge--draft {
  color: var(--text-muted);
  background: var(--surface-soft);
}

.post-time {
  color: var(--text-muted);
  font-size: var(--text-label);
}

.post-title {
  overflow: hidden;
  color: var(--text);
  font-size: 1.05rem;
  letter-spacing: -0.015em;
  text-overflow: ellipsis;
  white-space: nowrap;
  cursor: pointer;
  transition: color 0.2s ease;
}

.post-title:hover {
  color: var(--accent);
}

.post-desc {
  display: -webkit-box;
  overflow: hidden;
  margin: 0;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.65;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
}

.post-meta {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.4rem 0.9rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.post-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.3rem;
}

.tag-chip {
  padding: 0.12rem 0.5rem;
  border-radius: 999px;
  color: var(--text-soft);
  background: var(--surface-soft);
  font-size: 0.72rem;
}

/* ---- 圆形操作按钮 ---- */

.row-actions {
  display: flex;
  flex: 0 0 auto;
  gap: 0.45rem;
}

.round-action {
  display: grid;
  width: 2.25rem;
  height: 2.25rem;
  place-items: center;
  border: 0;
  border-radius: 999px;
  color: var(--text-muted);
  background: var(--surface-soft);
  transition:
    color 0.2s ease,
    background-color 0.2s ease;
}

.round-action:hover:not(:disabled) {
  color: var(--accent-contrast);
  background: var(--accent);
}

.round-action--accent:hover:not(:disabled) {
  color: var(--accent-contrast);
  background: var(--accent-hover);
}

.round-action--danger:hover:not(:disabled) {
  color: #ffffff;
  background: var(--danger);
}

.round-action:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

@media (max-width: 640px) {
  .post-card {
    flex-direction: column;
  }

  .row-actions {
    width: 100%;
  }

  .row-actions .round-action {
    flex: 1;
  }

  .posts-toolbar {
    flex-wrap: wrap;
  }

  .filter-tabs {
    overflow-x: auto;
  }
}
</style>
