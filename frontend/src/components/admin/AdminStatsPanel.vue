<script setup lang="ts">
import AppLink from '@/components/AppLink.vue'
import { computed, onMounted, ref } from 'vue'
import { getAdminStats } from '@/services/admin'
import type { StatsSummary } from '@/types/blog'

const stats = ref<StatsSummary | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

const cards = computed(() => {
  if (!stats.value) return []

  return [
    { label: '已发布文章', value: stats.value.publishedPosts },
    { label: '草稿', value: stats.value.draftPosts },
    { label: '总阅读量', value: stats.value.totalViews },
    { label: '待审核评论', value: stats.value.pendingComments },
  ]
})

async function load() {
  loading.value = true
  error.value = null

  try {
    stats.value = await getAdminStats()
  } catch (loadError) {
    error.value = loadError instanceof Error ? loadError.message : '加载统计失败'
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div class="stats-wrap">
    <div v-if="loading" class="admin-panel">
      <p class="panel-state">加载中…</p>
    </div>

    <p v-else-if="error" class="admin-panel panel-state panel-state--error">{{ error }}</p>

    <template v-else-if="stats">
      <div class="stat-cards">
        <div v-for="card in cards" :key="card.label" class="stat-card">
          <span class="stat-value">{{ card.value }}</span>
          <span class="stat-label">{{ card.label }}</span>
        </div>
      </div>

      <section class="admin-panel">
        <div class="panel-heading table-heading">
          <span>POSTS · 阅读与评论</span>
          <button class="action action--secondary action--small" type="button" :disabled="loading" @click="load">
            刷新
          </button>
        </div>

        <div class="table-scroll">
          <table class="stat-table">
            <thead>
              <tr>
                <th>文章</th>
                <th>状态</th>
                <th class="num">阅读</th>
                <th class="num">已通过评论</th>
                <th class="num">待审核</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="post in stats.posts" :key="post.slug">
                <td class="post-title" :title="post.title">
                  <AppLink :href="`/posts/${post.slug}`">{{ post.title }}</AppLink>
                </td>
                <td>
                  <span class="status-badge" :class="`status-badge--${post.status}`">
                    {{ post.status === 'published' ? '已发布' : '草稿' }}
                  </span>
                </td>
                <td class="num">{{ post.views }}</td>
                <td class="num">{{ post.approvedComments }}</td>
                <td class="num">{{ post.pendingComments || '—' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <p class="comment-summary">
        评论合计：{{ stats.approvedComments }} 已通过 · {{ stats.pendingComments }} 待审核 ·
        {{ stats.rejectedComments }} 已拒绝
      </p>
    </template>
  </div>
</template>

<style scoped>
.stat-cards {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 0.75rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  padding: 1rem 1.1rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background:
    linear-gradient(120deg, var(--accent-soft), transparent 62%),
    var(--surface);
}

.stat-value {
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 1.5rem;
  font-weight: 600;
  letter-spacing: -0.03em;
}

.stat-label {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

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

.panel-state {
  margin: 0;
  padding: 1.5rem 1.25rem;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.panel-state--error {
  color: var(--danger);
}

.table-scroll {
  overflow-x: auto;
}

.stat-table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--text-small);
}

.stat-table th {
  padding: 0.65rem 1.25rem;
  border-bottom: 1px solid var(--border);
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  font-weight: 500;
  text-align: left;
  white-space: nowrap;
}

.stat-table td {
  padding: 0.7rem 1.25rem;
  border-bottom: 1px solid var(--border);
  color: var(--text-soft);
}

.stat-table tbody tr:last-child td {
  border-bottom: 0;
}

.stat-table .num {
  text-align: right;
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.post-title {
  max-width: 24rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.post-title a {
  color: var(--text);
}

.post-title a:hover {
  color: var(--accent);
}

.status-badge {
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

.comment-summary {
  margin: 1rem 0.25rem 0;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

@media (max-width: 720px) {
  .stat-cards {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
