<script setup lang="ts">
import { formatDate, formatIndex } from '@/utils/format'
import type { Post } from '@/types/blog'

defineProps<{
  post: Post
  index: number
  featured?: boolean
}>()
</script>

<template>
  <article :class="['post-card', { 'post-card--featured': featured }]">
    <a class="post-link" :href="`#/posts/${post.slug}`">
      <div class="post-top">
        <span class="post-index">{{ formatIndex(index) }}</span>
        <div class="post-meta">
          <time :datetime="post.publishedAt">{{ formatDate(post.publishedAt) }}</time>
          <span class="meta-divider" aria-hidden="true">/</span>
          <span>{{ post.readingMinutes }} MIN</span>
        </div>
        <span v-if="featured" class="featured-label">FEATURED</span>
      </div>

      <h3 class="post-title">{{ post.title }}</h3>
      <p class="post-excerpt">{{ post.description }}</p>

      <div class="post-bottom">
        <ul class="post-tags" aria-label="文章标签">
          <li v-for="tag in post.tags" :key="tag">#{{ tag }}</li>
        </ul>
        <span class="post-arrow" aria-hidden="true">
          <svg
            viewBox="0 0 24 24"
            width="16"
            height="16"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M7 17 17 7M8 7h9v9" />
          </svg>
        </span>
      </div>
    </a>
  </article>
</template>

<style scoped>
.post-card {
  position: relative;
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  background: var(--surface);
  transition:
    transform 0.24s ease,
    border-color 0.24s ease,
    box-shadow 0.24s ease,
    background-color 0.24s ease;
}

.post-card::after {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, var(--accent), transparent 72%);
  content: '';
  opacity: 0;
  transform: scaleX(0.35);
  transform-origin: left;
  transition:
    opacity 0.24s ease,
    transform 0.24s ease;
}

.post-card:hover {
  border-color: var(--accent-line);
  box-shadow: var(--shadow-sm);
  transform: translateY(-2px);
}

.post-card:hover::after {
  opacity: 1;
  transform: scaleX(1);
}

.post-card--featured {
  border-color: var(--accent-line);
  background:
    linear-gradient(135deg, var(--accent-soft), transparent 58%),
    var(--surface);
}

.post-card--featured::after {
  opacity: 1;
  transform: scaleX(1);
}

.post-link {
  display: block;
  padding: 22px 24px 20px;
}

.post-top {
  display: flex;
  align-items: center;
  gap: 12px;
}

.post-index {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 0.7rem;
  letter-spacing: 0.04em;
}

.post-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.68rem;
  letter-spacing: 0.04em;
}

.meta-divider {
  color: var(--border-strong);
}

.featured-label {
  margin-left: auto;
  padding: 3px 7px;
  border: 1px solid var(--accent-line);
  border-radius: 4px;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 0.58rem;
  letter-spacing: 0.1em;
  line-height: 1.4;
}

.post-title {
  margin-top: 14px;
  color: var(--text);
  font-size: clamp(1.2rem, 2.2vw, 1.48rem);
  letter-spacing: -0.035em;
  transition: color 0.2s ease;
}

.post-card:hover .post-title {
  color: var(--accent);
}

.post-excerpt {
  max-width: 62ch;
  margin-top: 8px;
  color: var(--text-muted);
  font-size: 0.9rem;
  line-height: 1.72;
}

.post-bottom {
  display: flex;
  margin-top: 18px;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.post-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  list-style: none;
}

.post-tags li {
  padding: 2px 7px;
  border-radius: 4px;
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: 0.64rem;
  letter-spacing: 0.01em;
}

.post-arrow {
  display: grid;
  width: 30px;
  height: 30px;
  flex: 0 0 auto;
  place-items: center;
  border: 1px solid var(--border);
  border-radius: 50%;
  color: var(--text-muted);
  transition:
    color 0.24s ease,
    border-color 0.24s ease,
    background-color 0.24s ease,
    transform 0.24s ease;
}

.post-card:hover .post-arrow {
  border-color: var(--accent);
  color: var(--accent-contrast);
  background: var(--accent);
  transform: rotate(6deg);
}

@media (max-width: 560px) {
  .post-link {
    padding: 20px 18px 18px;
  }

  .post-excerpt {
    font-size: 0.86rem;
  }

  .post-bottom {
    margin-top: 16px;
  }

  .post-arrow {
    display: none;
  }
}
</style>
