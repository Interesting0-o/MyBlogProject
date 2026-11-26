<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import AppLink from '@/components/AppLink.vue'
import PostComments from '@/components/PostComments.vue'
import { fetchPosts } from '@/services/posts'
import type { Post } from '@/types/blog'
import { formatDate } from '@/utils/format'

const props = defineProps<{
  slug: string
}>()

const posts = ref<Post[]>([])
const loading = ref(true)
const loadError = ref(false)
const progress = ref(0)
const copyState = ref<'idle' | 'copied' | 'failed'>('idle')
let copyTimer: ReturnType<typeof setTimeout> | undefined

const currentPost = computed(() => posts.value.find((post) => post.slug === props.slug) ?? null)
const postIndex = computed(() =>
  currentPost.value
    ? posts.value.findIndex((post) => post.slug === currentPost.value?.slug)
    : -1,
)
const previousPost = computed<Post | null>(() => {
  if (postIndex.value <= 0) return null
  return posts.value[postIndex.value - 1] ?? null
})
const nextPost = computed<Post | null>(() => {
  if (postIndex.value < 0) return null
  return posts.value[postIndex.value + 1] ?? null
})
const copyLabel = computed(() => {
  if (copyState.value === 'copied') return '已复制'
  if (copyState.value === 'failed') return '复制失败'
  return '复制链接'
})

function updateProgress() {
  const maxScroll = document.documentElement.scrollHeight - window.innerHeight
  progress.value =
    maxScroll > 0 ? Math.min(100, Math.max(0, (window.scrollY / maxScroll) * 100)) : 0
}

async function loadPost() {
  loading.value = true
  loadError.value = false

  try {
    posts.value = await fetchPosts()
  } catch {
    loadError.value = true
  } finally {
    loading.value = false
  }

  await nextTick()
  updateProgress()
}

async function copyLink() {
  try {
    if (!navigator.clipboard) throw new Error('Clipboard unavailable')
    await navigator.clipboard.writeText(window.location.href)
    copyState.value = 'copied'
  } catch {
    copyState.value = 'failed'
  }

  if (copyTimer) clearTimeout(copyTimer)
  copyTimer = setTimeout(() => {
    copyState.value = 'idle'
  }, 1800)
}

watch(currentPost, (post) => {
  if (post) document.title = `${post.title} · sudo4`
})

watch(
  () => props.slug,
  () => {
    loadPost()
  },
  { immediate: true },
)

onMounted(() => {
  window.addEventListener('scroll', updateProgress, { passive: true })
  window.addEventListener('resize', updateProgress)
})

onBeforeUnmount(() => {
  window.removeEventListener('scroll', updateProgress)
  window.removeEventListener('resize', updateProgress)
  if (copyTimer) clearTimeout(copyTimer)
})
</script>

<template>
  <section class="post-page">
    <div
      v-if="currentPost"
      class="reading-progress"
      :style="{ width: `${progress}%` }"
      aria-hidden="true"
    ></div>

    <div class="post-container">
      <AppLink class="back-link" href="/">
        <span aria-hidden="true">←</span>
        cd /posts
      </AppLink>

      <div v-if="loading" class="post-loading" aria-busy="true">
        <span class="loading-command">❯ cat post.md</span>
        <span class="loading-line loading-line--title"></span>
        <span class="loading-line"></span>
        <span class="loading-line loading-line--short"></span>
      </div>

      <div v-else-if="loadError" class="post-missing">
        <span class="missing-code">[ERR]</span>
        <h1>文章加载失败</h1>
        <p>数据线路似乎断开了，请稍后再试。</p>
        <AppLink class="button-link" href="/">返回首页</AppLink>
      </div>

      <div v-else-if="!currentPost" class="post-missing">
        <span class="missing-code">404</span>
        <h1>这篇文章不见了</h1>
        <p>它可能被移动、重命名，或者从来没有存在过。</p>
        <AppLink class="button-link" href="/">cd ~ →</AppLink>
      </div>

      <template v-else>
        <header class="post-header">
          <div class="post-kicker">
            <span>ARTICLE</span>
            <span class="kicker-sep" aria-hidden="true">/</span>
            <span>{{ currentPost.readingMinutes }} MIN READ</span>
          </div>

          <h1 :title="currentPost.title">{{ currentPost.title }}</h1>
          <p class="post-lead">{{ currentPost.description }}</p>

          <div class="post-meta-row">
            <div class="post-meta-items">
              <span>
                <span class="meta-prompt" aria-hidden="true">❯</span>
                <time :datetime="currentPost.publishedAt">
                  {{ formatDate(currentPost.publishedAt) }}
                </time>
              </span>
              <span class="meta-divider" aria-hidden="true">/</span>
              <span>by sudo4</span>
              <template v-if="currentPost.updatedAt">
                <span class="meta-divider" aria-hidden="true">/</span>
                <span>updated {{ formatDate(currentPost.updatedAt) }}</span>
              </template>
            </div>

            <button class="copy-button" type="button" @click="copyLink">
              <svg
                v-if="copyState !== 'copied'"
                viewBox="0 0 24 24"
                width="14"
                height="14"
                fill="none"
                stroke="currentColor"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="M10 13a5 5 0 0 0 7.1 0l2-2a5 5 0 0 0-7.1-7.1l-1.1 1.1" />
                <path d="M14 11a5 5 0 0 0-7.1 0l-2 2A5 5 0 0 0 12 20.1l1.1-1.1" />
              </svg>
              <svg
                v-else
                viewBox="0 0 24 24"
                width="14"
                height="14"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="m5 12 4 4L19 6" />
              </svg>
              {{ copyLabel }}
            </button>
          </div>

          <ul class="post-tags" aria-label="文章标签">
            <li v-for="tag in currentPost.tags" :key="tag">#{{ tag }}</li>
          </ul>
        </header>

        <div class="post-divider" aria-hidden="true">
          <span></span>
          <span class="divider-label">BEGIN</span>
          <span></span>
        </div>

        <article class="prose" v-html="currentPost.content"></article>

        <div class="post-end">
          <span class="end-line" aria-hidden="true"></span>
          <span>EOF</span>
          <span class="end-line" aria-hidden="true"></span>
        </div>

        <nav class="post-nav" aria-label="文章导航">
          <AppLink v-if="previousPost" class="post-nav-item" :href="`/posts/${previousPost.slug}`">
            <span class="post-nav-label">← 较新一篇</span>
            <span class="post-nav-title">{{ previousPost.title }}</span>
          </AppLink>
          <div v-else class="post-nav-item post-nav-item--empty">
            <span class="post-nav-label">← 较新一篇</span>
            <span class="post-nav-title">已经是最新文章</span>
          </div>

          <AppLink v-if="nextPost" class="post-nav-item post-nav-item--next" :href="`/posts/${nextPost.slug}`">
            <span class="post-nav-label">较早一篇 →</span>
            <span class="post-nav-title">{{ nextPost.title }}</span>
          </AppLink>
          <div v-else class="post-nav-item post-nav-item--empty post-nav-item--next">
            <span class="post-nav-label">较早一篇 →</span>
            <span class="post-nav-title">已经读到最早一篇</span>
          </div>
        </nav>

        <PostComments :slug="currentPost.slug" />
      </template>
    </div>
  </section>
</template>

<style scoped>
.post-page {
  padding-top: 36px;
}

.reading-progress {
  position: fixed;
  top: var(--header-height);
  left: 0;
  z-index: 30;
  height: 2px;
  background: var(--accent);
  box-shadow: 0 0 12px var(--accent-glow);
  transition: width 0.1s linear;
}

.post-container {
  width: min(780px, calc(100% - 48px));
  margin-inline: auto;
}

.back-link {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  margin-bottom: clamp(36px, 6vw, 64px);
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  transition: color 0.2s ease;
}

.back-link:hover {
  color: var(--accent);
}

.post-header h1 {
  margin-top: 18px;
  overflow: hidden;
  font-size: clamp(1.45rem, 3.4vw, 2.35rem);
  font-weight: 650;
  letter-spacing: -0.04em;
  line-height: 1.25;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.post-kicker {
  display: flex;
  align-items: center;
  gap: 9px;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.07em;
}

.kicker-sep {
  color: var(--border-strong);
}

.post-lead {
  max-width: 60ch;
  margin-top: 22px;
  color: var(--text-soft);
  font-size: clamp(var(--text-lead), 1.5vw, 1.25rem);
  line-height: 1.85;
}

.post-meta-row {
  display: flex;
  margin-top: 30px;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding-block: 14px;
  border-top: 1px solid var(--border);
  border-bottom: 1px solid var(--border);
}

.post-meta-items {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 9px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.meta-prompt {
  margin-right: 5px;
  color: var(--accent);
}

.meta-divider {
  color: var(--border-strong);
}

.copy-button {
  display: inline-flex;
  min-height: 32px;
  align-items: center;
  gap: 6px;
  padding: 5px 9px;
  border: 1px solid var(--border);
  border-radius: 5px;
  color: var(--text-muted);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  white-space: nowrap;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.copy-button:hover {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.post-tags {
  display: flex;
  margin-top: 18px;
  flex-wrap: wrap;
  gap: 7px;
  list-style: none;
}

.post-tags li {
  padding: 3px 8px;
  border-radius: 4px;
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.post-divider {
  display: flex;
  margin: clamp(42px, 7vw, 64px) 0 36px;
  align-items: center;
  gap: 16px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.12em;
}

.post-divider span:not(.divider-label) {
  height: 1px;
  flex: 1;
  background: var(--border);
}

.divider-label {
  color: var(--accent);
}

.prose {
  color: var(--text-soft);
  font-size: 1.0625rem;
  line-height: 1.92;
}

.prose :deep(p) {
  margin: 1.25em 0;
}

.prose :deep(h2) {
  margin: 2.7em 0 0.85em;
  color: var(--text);
  font-size: 1.36rem;
  font-weight: 600;
  letter-spacing: -0.03em;
  line-height: 1.35;
}

.prose :deep(h3) {
  margin: 2.2em 0 0.7em;
  color: var(--text);
  font-size: 1.1rem;
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1.45;
}

.prose :deep(a) {
  color: var(--accent);
  text-decoration: underline;
  text-decoration-color: var(--accent-line);
  text-underline-offset: 3px;
}

.prose :deep(strong) {
  color: var(--text);
  font-weight: 600;
}

.prose :deep(ul) {
  display: grid;
  margin: 1.15em 0;
  gap: 0.45em;
  list-style: none;
}

.prose :deep(li) {
  position: relative;
  padding-left: 1.35em;
}

.prose :deep(li::before) {
  position: absolute;
  top: 0.12em;
  left: 0.1em;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 0.75em;
  content: '▸';
}

.prose :deep(ol) {
  margin: 1.15em 0;
  padding-left: 1.55em;
  list-style: decimal;
}

.prose :deep(ol li) {
  margin: 0.45em 0;
  padding-left: 0.15em;
}

.prose :deep(ol li::before) {
  content: none;
}

.prose :deep(ol li::marker) {
  color: var(--accent);
  font-family: var(--font-mono);
}

.prose :deep(hr) {
  margin: 2.6em 0;
  border: 0;
  border-top: 1px solid var(--border);
}

.prose :deep(img) {
  display: block;
  max-width: 100%;
  height: auto;
  margin: 1.8em auto;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
}

.prose :deep(blockquote) {
  margin: 2em 0;
  padding: 18px 22px;
  border-left: 2px solid var(--accent);
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
  color: var(--text);
  background: var(--accent-soft);
}

.prose :deep(blockquote p) {
  margin: 0;
}

.prose :deep(code) {
  padding: 0.12em 0.38em;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: 0.84em;
}

.prose :deep(pre) {
  margin: 1.7em 0;
  overflow-x: auto;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--code-bg);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.03);
}

.prose :deep(pre code) {
  display: block;
  padding: 20px 22px;
  border: 0;
  color: var(--code-text);
  background: transparent;
  font-size: var(--text-small);
  line-height: 1.75;
}

.post-end {
  display: flex;
  margin-top: clamp(48px, 8vw, 76px);
  align-items: center;
  gap: 16px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.14em;
}

.end-line {
  height: 1px;
  flex: 1;
  background: var(--border);
}

.post-nav {
  display: grid;
  margin-top: 48px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}

.post-nav-item {
  display: flex;
  min-height: 105px;
  flex-direction: column;
  justify-content: space-between;
  gap: 16px;
  padding: 18px 20px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
  transition:
    border-color 0.2s ease,
    background-color 0.2s ease,
    transform 0.2s ease;
}

a.post-nav-item:hover {
  border-color: var(--accent-line);
  background: var(--accent-soft);
  transform: translateY(-2px);
}

.post-nav-item--next {
  text-align: right;
}

.post-nav-label {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.post-nav-title {
  color: var(--text-soft);
  font-size: var(--text-small);
  font-weight: 500;
  line-height: 1.55;
}

a.post-nav-item:hover .post-nav-title {
  color: var(--accent);
}

.post-nav-item--empty {
  opacity: 0.52;
}

.post-loading {
  display: flex;
  min-height: 420px;
  flex-direction: column;
  gap: 18px;
  padding-top: 12px;
}

.loading-command {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.loading-line {
  width: 100%;
  height: 13px;
  border-radius: 4px;
  background: var(--surface-soft);
  animation: loading-pulse 1.4s ease-in-out infinite;
}

.loading-line--title {
  width: 72%;
  height: 42px;
  margin-top: 22px;
}

.loading-line--short {
  width: 52%;
}

.post-missing {
  display: flex;
  min-height: 480px;
  flex-direction: column;
  align-items: flex-start;
  justify-content: center;
}

.missing-code {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.12em;
}

.post-missing h1 {
  margin-top: 18px;
  font-size: clamp(2rem, 5vw, 3rem);
}

.post-missing p {
  margin-top: 14px;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.button-link {
  display: inline-flex;
  margin-top: 26px;
  padding: 9px 15px;
  border: 1px solid var(--accent-line);
  border-radius: var(--radius-sm);
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.button-link:hover {
  color: var(--accent-contrast);
  background: var(--accent);
}

@keyframes loading-pulse {
  0%,
  100% {
    opacity: 0.5;
  }

  50% {
    opacity: 1;
  }
}

@media (max-width: 720px) {
  .post-container {
    width: min(100% - 36px, 780px);
  }

  .post-page {
    padding-top: 24px;
  }

  .back-link {
    margin-bottom: 36px;
  }

  .post-meta-row {
    align-items: flex-start;
    flex-direction: column;
    gap: 12px;
  }

  .post-nav {
    grid-template-columns: 1fr;
  }

  .post-nav-item--next {
    text-align: left;
  }

  .post-header h1 {
    margin-top: 14px;
  }

  .prose {
    font-size: var(--text-body);
  }

  .prose :deep(h2) {
    font-size: 1.22rem;
  }
}
</style>
