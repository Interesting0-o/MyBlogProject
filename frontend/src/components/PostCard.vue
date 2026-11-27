<script setup lang="ts">
import { formatMonthDay } from '@/utils/format'
import type { Post } from '@/types/blog'

defineProps<{
  post: Post
}>()
</script>

<template>
  <article class="post-card">
    <a class="post-link" :href="`#/posts/${post.slug}`">
      <h3 class="post-title" :title="post.title">{{ post.title }}</h3>

      <div class="post-meta-row">
        <div class="post-meta">
          <time :datetime="post.publishedAt">{{ formatMonthDay(post.publishedAt) }}</time>
          <span class="meta-divider" aria-hidden="true">/</span>
          <span>{{ post.readingMinutes }} MIN</span>
        </div>

        <ul class="post-tags" aria-label="文章标签">
          <li v-for="tag in post.tags" :key="tag">#{{ tag }}</li>
        </ul>
      </div>
    </a>
  </article>
</template>

<style scoped>
.post-card {
  transition: color 0.2s ease;
}

.post-link {
  display: block;
  padding: 10px 0;
}

.post-title {
  overflow: hidden;
  color: var(--text);
  font-size: 1rem;
  letter-spacing: -0.02em;
  text-overflow: ellipsis;
  transition: color 0.2s ease;
  white-space: nowrap;
}

.post-card:hover .post-title {
  color: var(--accent);
}

.post-meta-row {
  display: flex;
  margin-top: 0.35rem;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.35rem 1rem;
}

.post-meta {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.75rem;
  letter-spacing: 0.03em;
}

.meta-divider {
  color: var(--border-strong);
}

.post-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.65rem;
  list-style: none;
}

.post-tags li {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.72rem;
  opacity: 0.85;
}
</style>
