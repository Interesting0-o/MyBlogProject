<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import {
  deleteAdminComment,
  listAdminComments,
  setAdminCommentStatus,
} from '@/services/admin'
import type { Comment, CommentStatus } from '@/types/blog'

const comments = ref<Comment[]>([])
const filter = ref<CommentStatus | 'all'>('all')
const loading = ref(false)
const error = ref<string | null>(null)
const message = ref<string | null>(null)

const filters: { value: CommentStatus | 'all'; label: string }[] = [
  { value: 'all', label: '全部' },
  { value: 'pending', label: '待审核' },
  { value: 'approved', label: '已通过' },
  { value: 'rejected', label: '已拒绝' },
]

const pendingCount = computed(
  () => comments.value.filter((comment) => comment.status === 'pending').length,
)

const statusLabel: Record<CommentStatus, string> = {
  pending: '待审核',
  approved: '已通过',
  rejected: '已拒绝',
}

async function load() {
  loading.value = true
  error.value = null

  try {
    comments.value = await listAdminComments(filter.value)
  } catch (loadError) {
    error.value = loadError instanceof Error ? loadError.message : '加载评论失败'
  } finally {
    loading.value = false
  }
}

async function act(comment: Comment, status: CommentStatus) {
  message.value = null

  try {
    await setAdminCommentStatus(comment.id, status)
    message.value = status === 'approved' ? '已通过' : '已拒绝'
    await load()
  } catch (actionError) {
    error.value = actionError instanceof Error ? actionError.message : '操作失败'
  }
}

async function remove(comment: Comment) {
  if (!window.confirm(`删除「${comment.author}」的这条评论？`)) {
    return
  }

  message.value = null

  try {
    await deleteAdminComment(comment.id)
    message.value = '评论已删除'
    await load()
  } catch (deleteError) {
    error.value = deleteError instanceof Error ? deleteError.message : '删除失败'
  }
}

onMounted(load)
</script>

<template>
  <section class="admin-panel">
    <div class="panel-heading comments-heading">
      <span>COMMENTS · {{ pendingCount }} 待审核</span>
      <button class="action action--secondary action--small" type="button" :disabled="loading" @click="load">
        {{ loading ? '刷新中…' : '刷新' }}
      </button>
    </div>

    <div class="filter-bar">
      <button
        v-for="item in filters"
        :key="item.value"
        class="filter-chip"
        :class="{ active: filter === item.value }"
        type="button"
        @click="filter = item.value; load()"
      >
        {{ item.label }}
      </button>
    </div>

    <p v-if="error" class="panel-state panel-state--error">{{ error }}</p>
    <p v-else-if="loading && !comments.length" class="panel-state">加载中…</p>
    <p v-else-if="!comments.length" class="panel-state">没有符合条件的评论。</p>
    <p v-else-if="message" class="panel-state panel-state--ok">{{ message }}</p>

    <ul v-if="comments.length" class="comment-list">
      <li v-for="comment in comments" :key="comment.id" class="comment-row">
        <div class="comment-main">
          <div class="comment-head">
            <strong class="comment-author">{{ comment.author }}</strong>
            <span class="status-badge" :class="`status-badge--${comment.status}`">
              {{ statusLabel[comment.status] }}
            </span>
            <span class="comment-meta">{{ comment.createdAt }} · /posts/{{ comment.postSlug }}</span>
          </div>
          <p class="comment-content">{{ comment.content }}</p>
        </div>
        <div class="row-actions">
          <button
            v-if="comment.status !== 'approved'"
            class="action action--secondary action--small"
            type="button"
            @click="act(comment, 'approved')"
          >
            通过
          </button>
          <button
            v-if="comment.status !== 'rejected'"
            class="action action--secondary action--small"
            type="button"
            @click="act(comment, 'rejected')"
          >
            拒绝
          </button>
          <button class="action action--danger action--small" type="button" @click="remove(comment)">
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

.filter-bar {
  display: flex;
  gap: 0.4rem;
  padding: 0.75rem 1.25rem;
  border-bottom: 1px solid var(--border);
}

.filter-chip {
  padding: 0.3rem 0.65rem;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-soft);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.filter-chip:hover {
  border-color: var(--accent-line);
  color: var(--accent);
}

.filter-chip.active {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
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

.panel-state--ok {
  color: var(--accent);
}

.comment-list {
  margin: 0;
  padding: 0;
  list-style: none;
}

.comment-row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.9rem 1.25rem;
  border-bottom: 1px solid var(--border);
}

.comment-row:last-child {
  border-bottom: 0;
}

.comment-main {
  display: flex;
  min-width: 0;
  flex: 1;
  flex-direction: column;
  gap: 0.4rem;
}

.comment-head {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.comment-author {
  color: var(--text);
  font-size: var(--text-small);
}

.comment-meta {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.comment-content {
  margin: 0;
  color: var(--text-soft);
  font-size: var(--text-small);
  line-height: 1.7;
  white-space: pre-wrap;
}

.status-badge {
  padding: 0.15rem 0.45rem;
  border-radius: 4px;
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.status-badge--pending {
  color: var(--text-muted);
  background: var(--surface-soft);
}

.status-badge--approved {
  color: var(--accent);
  background: var(--accent-soft);
}

.status-badge--rejected {
  color: var(--danger);
  background: var(--danger-soft);
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

.action--danger {
  color: var(--danger);
}

.action--danger:hover {
  border-color: var(--danger-line);
  color: var(--danger);
  background: var(--danger-soft);
}

@media (max-width: 640px) {
  .comment-row {
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
