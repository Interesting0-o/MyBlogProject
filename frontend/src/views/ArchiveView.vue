<script setup lang="ts">
import AppLink from '@/components/AppLink.vue'
import { computed, onMounted, ref } from 'vue'
import { fetchPosts } from '@/services/posts'
import type { Post } from '@/types/blog'
import { formatMonthDay, getYear } from '@/utils/format'

const posts = ref<Post[]>([])
const loading = ref(true)
const loadError = ref(false)

const groupedPosts = computed(() => {
  const groups = new Map<string, Post[]>()

  for (const post of posts.value) {
    const year = getYear(post.publishedAt)
    const entries = groups.get(year)

    if (entries) {
      entries.push(post)
    } else {
      groups.set(year, [post])
    }
  }

  return Array.from(groups, ([year, entries]) => ({ year, entries }))
})

async function loadPosts() {
  loading.value = true
  loadError.value = false

  try {
    posts.value = await fetchPosts()
  } catch {
    loadError.value = true
  } finally {
    loading.value = false
  }
}

onMounted(loadPosts)
</script>

<template>
  <section class="archive-page page-with-glow container">
    <header class="page-header">
      <div class="page-kicker">ARCHIVE</div>
      <h1>全部归档<span class="accent">.</span></h1>
      <p>
        共 {{ loading ? '…' : posts.length }} 篇文章，按时间倒序排列。这里保存着所有已经写完的思考与折腾。
      </p>
    </header>

    <div v-if="loading" class="archive-loading" aria-busy="true">
      <div v-for="item in 4" :key="item" class="archive-skeleton">
        <span></span>
        <span></span>
      </div>
    </div>

    <div v-else-if="loadError" class="archive-empty">
      <span class="empty-command">[ERR]</span>
      <p>归档加载失败，请稍后再试。</p>
      <button type="button" @click="loadPosts">重新加载</button>
    </div>

    <div v-else class="archive-groups">
      <section v-for="group in groupedPosts" :key="group.year" class="archive-group">
        <div class="year-heading">
          <h2>{{ group.year }}</h2>
          <span>{{ group.entries.length }} entries</span>
          <span class="year-line" aria-hidden="true"></span>
        </div>

        <div class="archive-list">
          <a
            v-for="post in group.entries"
            :key="post.slug"
            class="archive-row"
            :href="`/posts/${post.slug}`"
          >
            <time :datetime="post.publishedAt">{{ formatMonthDay(post.publishedAt) }}</time>
            <span class="archive-title" :title="post.title">{{ post.title }}</span>
          </a>
        </div>
      </section>
    </div>

    <AppLink class="archive-back" href="/">
      <span aria-hidden="true">←</span>
      cd ~
    </AppLink>
  </section>
</template>

<style scoped>
.archive-page {
  width: min(920px, calc(100% - 96px));
  padding-top: clamp(1.75rem, 3.5vw, 3rem);
}

.page-header {
  padding-bottom: clamp(38px, 6vw, 56px);
  border-bottom: 1px solid var(--border);
}

.page-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.page-header h1 {
  margin-top: 15px;
  font-size: clamp(2.5rem, 6vw, 4rem);
  letter-spacing: -0.06em;
}

.accent {
  color: var(--accent);
}

.page-header > p {
  max-width: 52ch;
  margin-top: 18px;
  color: var(--text-muted);
  font-size: var(--text-body);
}

.archive-groups {
  display: grid;
  gap: clamp(56px, 7vw, 72px);
  padding-top: clamp(40px, 6vw, 56px);
}

.year-heading {
  display: flex;
  align-items: center;
  gap: 14px;
}

.year-heading h2 {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 1.15rem;
  font-weight: 500;
  letter-spacing: -0.03em;
}

.year-heading > span:not(.year-line) {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.year-line {
  height: 1px;
  flex: 1;
  background: var(--border);
}

.archive-list {
  display: grid;
  margin-top: 10px;
  gap: 2px;
}

.archive-row {
  display: grid;
  grid-template-columns: 56px minmax(0, 1fr);
  align-items: center;
  gap: 18px;
  padding: 9px 8px;
  border-radius: 6px;
  transition: background-color 0.2s ease;
}

.archive-row:hover {
  background: var(--accent-soft);
}

.archive-row time {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.archive-title {
  overflow: hidden;
  color: var(--text-soft);
  font-size: var(--text-body);
  font-weight: 500;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color 0.2s ease;
}

.archive-row:hover .archive-title {
  color: var(--accent);
}

.archive-loading {
  display: grid;
  padding-top: 36px;
}

.archive-skeleton {
  display: grid;
  grid-template-columns: 56px 1fr;
  gap: 18px;
  padding: 9px 8px;
}

.archive-skeleton span {
  height: 11px;
  border-radius: 4px;
  background: var(--surface-soft);
  animation: archive-pulse 1.4s ease-in-out infinite;
}

.archive-skeleton span:last-child {
  width: 60%;
}

.archive-empty {
  display: flex;
  min-height: 260px;
  flex-direction: column;
  align-items: flex-start;
  justify-content: center;
  color: var(--text-muted);
}

.empty-command {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.archive-empty p {
  margin-top: 12px;
}

.archive-empty button {
  margin-top: 18px;
  padding: 7px 12px;
  border: 1px solid var(--accent-line);
  border-radius: var(--radius-sm);
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.archive-back {
  display: inline-flex;
  margin-top: clamp(48px, 8vw, 72px);
  align-items: center;
  gap: 8px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.archive-back:hover {
  color: var(--accent);
}

@keyframes archive-pulse {
  0%,
  100% {
    opacity: 0.5;
  }

  50% {
    opacity: 1;
  }
}

@media (max-width: 720px) {
  .archive-page {
    width: calc(100% - 48px);
  }

  .archive-row {
    grid-template-columns: 48px minmax(0, 1fr);
    gap: 12px;
    padding-inline: 6px;
  }

  .archive-title {
    font-size: var(--text-small);
  }
}
</style>
