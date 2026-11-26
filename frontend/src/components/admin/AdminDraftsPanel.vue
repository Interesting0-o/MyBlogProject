<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import AppLink from '@/components/AppLink.vue'
import { navigate } from '@/composables/useRoute'
import { deleteAdminPost, listAdminPosts, updateAdminPost, type AdminPost } from '@/services/admin'
import { setEditingPost } from '@/stores/adminPost'

const drafts = ref<AdminPost[]>([])
const loading = ref(false)
const processing = ref<string | null>(null)
const error = ref<string | null>(null)
const message = ref<{ type: 'ok' | 'error'; text: string } | null>(null)

const sortedDrafts = computed(() =>
  [...drafts.value].sort((a, b) => b.publishedAt.localeCompare(a.publishedAt)),
)

function today(): string {
  return new Date().toISOString().slice(0, 10)
}

async function load() {
  loading.value = true
  error.value = null

  try {
    const posts = await listAdminPosts()
    drafts.value = posts.filter((post) => post.status === 'draft')
  } catch (loadError) {
    error.value = loadError instanceof Error ? loadError.message : '加载草稿失败'
  } finally {
    loading.value = false
  }
}

function continueEditing(draft: AdminPost) {
  setEditingPost(draft)
  navigate('/admin/compose')
}

async function publish(draft: AdminPost) {
  if (!window.confirm(`直接发布「${draft.title}」？`)) {
    return
  }

  processing.value = draft.slug
  error.value = null
  message.value = null

  try {
    await updateAdminPost(draft.slug, {
      slug: draft.slug,
      title: draft.title,
      description: draft.description,
      publishedAt: draft.publishedAt,
      updatedAt: today(),
      tags: draft.tags,
      readingMinutes: draft.readingMinutes,
      featured: Boolean(draft.featured),
      status: 'published',
      content: draft.content,
    })

    message.value = { type: 'ok', text: `「${draft.title}」已发布` }
    await load()
  } catch (publishError) {
    error.value = publishError instanceof Error ? publishError.message : '发布失败'
  } finally {
    processing.value = null
  }
}

async function remove(draft: AdminPost) {
  if (!window.confirm(`删除草稿「${draft.title}」？此操作不可恢复。`)) {
    return
  }

  processing.value = draft.slug
  error.value = null
  message.value = null

  try {
    await deleteAdminPost(draft.slug)
    message.value = { type: 'ok', text: `已删除草稿「${draft.title}」` }
    await load()
  } catch (deleteError) {
    error.value = deleteError instanceof Error ? deleteError.message : '删除失败'
  } finally {
    processing.value = null
  }
}

onMounted(load)
</script>

<template>
  <section class="admin-panel drafts-panel">
    <div class="panel-heading drafts-heading">
      <span>DRAFTS · {{ drafts.length }} 篇</span>
      <div class="heading-actions">
        <button class="action action--secondary action--small" type="button" :disabled="loading" @click="load">
          {{ loading ? '刷新中…' : '刷新' }}
        </button>
        <AppLink class="action action--publish action--small" href="/admin/compose">写新草稿</AppLink>
      </div>
    </div>

    <p v-if="message" class="form-message" :class="`form-message--${message.type}`">
      {{ message.text }}
    </p>

    <p v-if="error" class="panel-state panel-state--error">{{ error }}</p>
    <p v-else-if="loading && !drafts.length" class="panel-state">加载中…</p>
    <p v-else-if="!drafts.length" class="panel-state">
      没有草稿。在发表文章页把状态切换为「草稿」或点击「存为草稿」，保存的草稿会出现在这里。
    </p>

    <ul v-else class="draft-list">
      <li v-for="draft in sortedDrafts" :key="draft.slug" class="draft-row">
        <div class="draft-info">
          <strong class="draft-title" :title="draft.title">{{ draft.title }}</strong>
          <span class="draft-meta">
            /posts/{{ draft.slug }} · {{ draft.publishedAt }} · 最后更新
            {{ draft.updatedAt || draft.publishedAt }}<template v-if="draft.tags.length"> · {{ draft.tags.join(' / ') }}</template>
          </span>
          <span class="draft-preview">{{ draft.content.slice(0, 80) }}{{ draft.content.length > 80 ? '…' : '' }}</span>
        </div>
        <div class="row-actions">
          <button
            class="action action--secondary action--small"
            type="button"
            :disabled="processing === draft.slug"
            @click="continueEditing(draft)"
          >
            继续编辑
          </button>
          <button
            class="action action--publish action--small"
            type="button"
            :disabled="processing === draft.slug"
            @click="publish(draft)"
          >
            {{ processing === draft.slug ? '处理中…' : '直接发布' }}
          </button>
          <button
            class="action action--danger action--small"
            type="button"
            :disabled="processing === draft.slug"
            @click="remove(draft)"
          >
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
  line-height: 1.7;
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

.draft-list {
  margin: 0;
  padding: 0;
  list-style: none;
}

.draft-row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border);
}

.draft-row:last-child {
  border-bottom: 0;
}

.draft-info {
  display: flex;
  min-width: 0;
  flex: 1;
  flex-direction: column;
  gap: 0.35rem;
}

.draft-title {
  overflow: hidden;
  color: var(--text);
  font-size: var(--text-small);
  text-overflow: ellipsis;
  white-space: nowrap;
}

.draft-meta {
  overflow: hidden;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  text-overflow: ellipsis;
  white-space: nowrap;
}

.draft-preview {
  display: -webkit-box;
  overflow: hidden;
  color: var(--text-soft);
  font-size: var(--text-small);
  line-height: 1.6;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
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
  white-space: nowrap;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.action:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.action--secondary:hover:not(:disabled) {
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

.action--danger:hover:not(:disabled) {
  border-color: var(--danger-line);
  color: var(--danger);
  background: var(--danger-soft);
}

@media (max-width: 640px) {
  .draft-row {
    flex-direction: column;
  }

  .row-actions {
    width: 100%;
  }

  .row-actions .action {
    flex: 1;
  }
}
</style>
