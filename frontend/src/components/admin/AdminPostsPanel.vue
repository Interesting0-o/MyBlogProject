<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import AppLink from '@/components/AppLink.vue'
import { navigate } from '@/composables/useRoute'
import { deleteAdminPost, listAdminPosts, type AdminPost } from '@/services/admin'
import { setEditingPost } from '@/stores/adminPost'

const posts = ref<AdminPost[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const message = ref<{ type: 'ok' | 'error'; text: string } | null>(null)

const pendingHint = computed(() =>
  posts.value.filter((post) => post.status === 'draft').length,
)

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

async function remove(post: AdminPost) {
  if (!window.confirm(`删除文章「${post.title}」？此操作不可恢复。`)) {
    return
  }

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
  }
}

onMounted(load)
</script>

<template>
  <section class="admin-panel manage-panel">
    <div class="panel-heading manage-heading">
      <span>POSTS · {{ posts.length }} 篇<template v-if="pendingHint"> · {{ pendingHint }} 草稿</template></span>
      <div class="heading-actions">
        <button class="action action--secondary action--small" type="button" :disabled="loading" @click="load">
          {{ loading ? '刷新中…' : '刷新' }}
        </button>
        <AppLink class="action action--publish action--small" href="/admin/compose">发表新文章</AppLink>
      </div>
    </div>

    <p v-if="message" class="form-message" :class="`form-message--${message.type}`">
      {{ message.text }}
    </p>

    <p v-if="error" class="panel-state panel-state--error">{{ error }}</p>
    <p v-else-if="loading && !posts.length" class="panel-state">加载中…</p>
    <p v-else-if="!posts.length" class="panel-state">还没有文章，写第一篇吧。</p>

    <ul v-else class="posts-list">
      <li v-for="post in posts" :key="post.slug" class="post-row">
        <div class="post-info">
          <div class="post-title-line">
            <span class="status-badge" :class="`status-badge--${post.status}`">
              {{ post.status === 'published' ? '已发布' : '草稿' }}
            </span>
            <strong class="post-title" :title="post.title">{{ post.title }}</strong>
          </div>
          <span class="post-meta">
            /posts/{{ post.slug }} · {{ post.publishedAt }} · {{ post.views ?? 0 }} 次阅读<template v-if="post.tags.length"> · {{ post.tags.join(' / ') }}</template>
          </span>
        </div>
        <div class="row-actions">
          <button class="action action--secondary action--small" type="button" @click="edit(post)">
            编辑
          </button>
          <button class="action action--danger action--small" type="button" @click="remove(post)">
            删除
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
  border-radius: var(--radius-md);
  background: var(--surface);
}

.panel-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.85rem 1.25rem;
  border-bottom: 1px solid var(--border);
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  background: var(--surface-soft);
}

.heading-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.panel-state {
  margin: 0;
  padding: 1.5rem 1.25rem;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.panel-state--error {
  color: var(--danger);
}

.form-message {
  margin: 0.85rem 1.25rem 0;
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

.posts-list {
  margin: 0;
  padding: 0;
  list-style: none;
}

.post-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.9rem 1.25rem;
  border-bottom: 1px solid var(--border);
}

.post-row:last-child {
  border-bottom: 0;
}

.post-info {
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: 0.3rem;
}

.post-title-line {
  display: flex;
  align-items: center;
  gap: 0.6rem;
}

.status-badge {
  flex: 0 0 auto;
  padding: 0.15rem 0.45rem;
  border-radius: 4px;
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.status-badge--published {
  color: var(--accent);
  background: var(--accent-soft);
}

.status-badge--draft {
  color: var(--text-muted);
  background: var(--surface-soft);
}

.post-title {
  overflow: hidden;
  color: var(--text);
  font-size: var(--text-small);
  text-overflow: ellipsis;
  white-space: nowrap;
}

.post-meta {
  overflow: hidden;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-actions {
  display: flex;
  flex: 0 0 auto;
  gap: 0.5rem;
}

.action {
  display: inline-flex;
  min-height: 2rem;
  align-items: center;
  justify-content: center;
  padding: 0.3rem 0.6rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  color: var(--text-soft);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.action--secondary:hover {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.action--publish {
  border-color: transparent;
  color: var(--accent-contrast);
  background: var(--accent);
}

.action--danger {
  color: var(--danger);
}

.action--danger:hover {
  border-color: var(--danger-line);
  color: var(--danger);
  background: var(--danger-soft);
}

@media (max-width: 640px) {
  .post-row {
    flex-direction: column;
    align-items: flex-start;
  }

  .row-actions {
    width: 100%;
  }

  .row-actions .action {
    flex: 1;
  }
}
</style>
