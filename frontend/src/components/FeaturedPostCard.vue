<script setup lang="ts">
import AppLink from '@/components/AppLink.vue'
import { formatDate } from '@/utils/format'
import type { Post } from '@/types/blog'

defineProps<{
  post: Post
}>()
</script>

<template>
  <section class="featured-block" aria-labelledby="featured-post-title">
    <span class="featured-kicker">FEATURED</span>

    <AppLink class="featured-card" :href="`/posts/${post.slug}`">
      <div class="featured-meta">
        <time :datetime="post.publishedAt">{{ formatDate(post.publishedAt) }}</time>
        <span class="meta-divider" aria-hidden="true">/</span>
        <span>{{ post.readingMinutes }} MIN</span>
      </div>

      <h2 id="featured-post-title" class="featured-title" :title="post.title">{{ post.title }}</h2>
      <p class="featured-excerpt">{{ post.description }}</p>

      <div class="featured-footer">
        <ul class="featured-tags" aria-label="文章标签">
          <li v-for="tag in post.tags" :key="tag">#{{ tag }}</li>
        </ul>
        <span class="featured-cta">
          阅读全文
          <span aria-hidden="true">→</span>
        </span>
      </div>
    </AppLink>
  </section>
</template>

<style scoped>
.featured-block {
  margin-bottom: clamp(2.5rem, 5vw, 4rem);
}

.featured-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  font-weight: 500;
  letter-spacing: 0.06em;
}

.featured-card {
  display: block;
  margin-top: 0.9rem;
  padding: clamp(1.5rem, 3vw, 2.25rem);
  border: 1px solid var(--accent-line);
  border-radius: var(--radius-lg);
  background:
    linear-gradient(135deg, var(--accent-soft), transparent 62%),
    var(--surface);
  box-shadow: 0 16px 42px var(--accent-glow);
  transition:
    border-color 0.2s ease,
    transform 0.2s ease;
}

.featured-card:hover {
  border-color: var(--accent);
  transform: translateY(-2px);
}

.featured-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.03em;
}

.meta-divider {
  color: var(--border-strong);
}

.featured-title {
  margin-top: 0.8rem;
  overflow: hidden;
  color: var(--text);
  font-size: clamp(1.35rem, 2.2vw, 1.85rem);
  letter-spacing: -0.035em;
  line-height: 1.3;
  text-overflow: ellipsis;
  transition: color 0.2s ease;
  white-space: nowrap;
}

.featured-card:hover .featured-title {
  color: var(--accent);
}

.featured-excerpt {
  max-width: 62ch;
  margin-top: 0.7rem;
  color: var(--text-soft);
  font-size: var(--text-body);
  line-height: 1.8;
}

.featured-footer {
  display: flex;
  margin-top: 1.4rem;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.featured-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.8rem;
  list-style: none;
}

.featured-tags li {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  opacity: 0.85;
}

.featured-cta {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--accent);
  font-size: var(--text-small);
  white-space: nowrap;
}
</style>
