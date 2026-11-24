<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import FeaturedPostCard from '@/components/FeaturedPostCard.vue'
import PostCard from '@/components/PostCard.vue'
import ScrollCue from '@/components/ScrollCue.vue'
import { profile } from '@/content/profile'
import { fetchPosts } from '@/services/posts'
import type { Post } from '@/types/blog'
import { isTypingTarget } from '@/utils/dom'

const posts = ref<Post[]>([])
const loading = ref(true)
const loadError = ref(false)
const query = ref('')
const selectedTag = ref('')
const searchInput = ref<HTMLInputElement | null>(null)

const tags = computed(() => {
  return [...new Set(posts.value.flatMap((post) => post.tags))].sort((a, b) =>
    a.localeCompare(b, 'zh-CN'),
  )
})

const featuredPost = computed(() => posts.value.find((post) => post.featured) ?? null)
const recentPosts = computed(() => posts.value.slice(0, 3))

function shortHash(value: string): string {
  let hash = 2166136261

  for (const character of value) {
    hash ^= character.codePointAt(0) ?? 0
    hash = Math.imul(hash, 16777619)
  }

  return (hash >>> 0).toString(16).padStart(6, '0').slice(0, 6)
}

const filteredPosts = computed(() => {
  const keyword = query.value.trim().toLowerCase()
  const tag = selectedTag.value

  return posts.value.filter((post) => {
    const matchesTag = !tag || post.tags.includes(tag)
    const matchesKeyword =
      !keyword ||
      [post.title, post.description, ...post.tags].some((value) =>
        value.toLowerCase().includes(keyword),
      )

    return matchesTag && matchesKeyword
  })
})

const hasFilters = computed(() => Boolean(query.value.trim() || selectedTag.value))
const postCountLabel = computed(() =>
  loading.value ? '--' : String(posts.value.length).padStart(2, '0'),
)
const tagCountLabel = computed(() =>
  loading.value ? '--' : String(tags.value.length).padStart(2, '0'),
)

function selectTag(tag: string) {
  selectedTag.value = selectedTag.value === tag ? '' : tag
}

function resetFilters() {
  query.value = ''
  selectedTag.value = ''
}

function scrollToPosts() {
  const behavior = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    ? 'auto'
    : 'smooth'

  document.getElementById('writing')?.scrollIntoView({ behavior })
}

function handleShortcut(event: KeyboardEvent) {
  if (
    event.key !== '/' ||
    event.metaKey ||
    event.ctrlKey ||
    event.altKey ||
    isTypingTarget(event.target)
  ) {
    return
  }

  event.preventDefault()
  searchInput.value?.focus()
}

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

onMounted(() => {
  loadPosts()
  window.addEventListener('keydown', handleShortcut)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleShortcut)
})
</script>

<template>
  <div class="home">
    <section class="hero container">
      <div class="hero-copy">
        <span class="status-badge">
          <span class="status-dot" aria-hidden="true"></span>
          open to build · 持续构建
        </span>

        <h1>
          在终端之外，<br />
          <span class="hero-accent">记录代码与生活。</span>
        </h1>

        <p class="hero-lead">
          这里是 sudo4 的数字花园。写下后端工程、系统折腾，以及那些值得被时间留下的思考。
        </p>

        <div class="hero-actions">
          <button class="button button--primary" type="button" @click="scrollToPosts">
            浏览文章
            <span aria-hidden="true">↓</span>
          </button>
          <a class="button button--ghost" href="#/about">
            关于我
            <span aria-hidden="true">↗</span>
          </a>
        </div>

        <dl class="hero-stats">
          <div>
            <dt>POSTS</dt>
            <dd>{{ postCountLabel }}</dd>
          </div>
          <div>
            <dt>TOPICS</dt>
            <dd>{{ tagCountLabel }}</dd>
          </div>
        </dl>
      </div>

      <aside class="hero-terminal" aria-label="终端风格的个人介绍">
        <div class="terminal-bar">
          <span class="terminal-dots" aria-hidden="true">
            <i></i>
            <i></i>
            <i></i>
          </span>
          <span class="terminal-title">visitor@sudo4: ~</span>
          <span class="terminal-shell">zsh</span>
        </div>

        <div class="terminal-body">
          <div class="terminal-block">
            <p class="terminal-line"><span class="prompt">❯</span> whoami</p>
            <p class="terminal-output terminal-output--identity">
              <span>{{ profile.username }}</span>
              <span>— backend / infra / writing</span>
            </p>
          </div>

          <div class="terminal-block">
            <p class="terminal-line"><span class="prompt">❯</span> git log --oneline -3</p>
            <div v-if="recentPosts.length" class="terminal-log">
              <a
                v-for="post in recentPosts"
                :key="post.slug"
                class="terminal-log-line"
                :href="`#/posts/${post.slug}`"
              >
                <span class="terminal-log-hash">{{ shortHash(post.slug) }}</span>
                <span class="terminal-log-title">{{ post.title }}</span>
              </a>
            </div>
            <p v-else class="terminal-log-empty">
              {{ loadError ? 'connection lost' : loading ? 'loading…' : 'no commits yet' }}
            </p>
          </div>

          <p class="terminal-line terminal-line--caret">
            <span class="prompt">❯</span>
            <span class="terminal-caret" aria-hidden="true"></span>
          </p>
        </div>

        <div class="terminal-footer">
          <span>UTF-8</span>
          <span>git:main</span>
        </div>
      </aside>

      <ScrollCue target="writing" aria-label="滚动到文章列表" />
    </section>

    <section id="writing" class="writing container">
      <div class="writing-main">
        <FeaturedPostCard v-if="featuredPost" :post="featuredPost" />

        <div class="section-head">
          <div>
            <span class="section-kicker">// LATEST POSTS</span>
            <h2>最近写下的</h2>
          </div>

          <div class="search-box">
            <svg
              viewBox="0 0 24 24"
              width="16"
              height="16"
              fill="none"
              stroke="currentColor"
              stroke-width="1.8"
              stroke-linecap="round"
              aria-hidden="true"
            >
              <circle cx="11" cy="11" r="7" />
              <path d="m16.2 16.2 4 4" />
            </svg>
            <label class="sr-only" for="post-search">搜索文章</label>
            <input
              id="post-search"
              ref="searchInput"
              v-model="query"
              type="search"
              placeholder="搜索文章…"
              autocomplete="off"
              spellcheck="false"
              @keydown.esc="query = ''"
            />
            <kbd v-if="!query" aria-hidden="true">/</kbd>
            <button
              v-else
              class="search-clear"
              type="button"
              aria-label="清除搜索"
              @click="query = ''"
            >
              ×
            </button>
          </div>
        </div>

        <div class="tag-filter" aria-label="按标签筛选">
          <button
            type="button"
            :class="{ active: selectedTag === '' }"
            :aria-pressed="selectedTag === ''"
            @click="selectedTag = ''"
          >
            all
          </button>
          <button
            v-for="tag in tags"
            :key="tag"
            type="button"
            :class="{ active: selectedTag === tag }"
            :aria-pressed="selectedTag === tag"
            @click="selectTag(tag)"
          >
            #{{ tag }}
          </button>
        </div>

        <div class="result-meta">
          <span>{{ filteredPosts.length }} 篇文章</span>
          <button v-if="hasFilters" type="button" @click="resetFilters">清除筛选</button>
        </div>

        <div v-if="loading" class="post-list" aria-busy="true" aria-label="正在加载文章">
          <div v-for="item in 3" :key="item" class="skeleton-card">
            <span class="skeleton-line skeleton-line--short"></span>
            <span class="skeleton-line skeleton-line--title"></span>
            <span class="skeleton-line"></span>
            <span class="skeleton-line skeleton-line--medium"></span>
          </div>
        </div>

        <div v-else-if="loadError" class="empty-state">
          <span class="empty-command">❯ fetch ./posts</span>
          <h3>文章加载失败</h3>
          <p>数据线路似乎断开了，请稍后再试。</p>
          <button class="button button--ghost" type="button" @click="loadPosts">重新加载</button>
        </div>

        <div v-else-if="filteredPosts.length === 0" class="empty-state">
          <span class="empty-command">❯ grep -r "{{ query || selectedTag }}" ./posts</span>
          <h3>0 results found</h3>
          <p>没有找到匹配的文章。换一个关键词，或者清除当前筛选。</p>
          <button class="button button--ghost" type="button" @click="resetFilters">
            清除筛选
          </button>
        </div>

        <div v-else class="post-list">
          <PostCard
            v-for="post in filteredPosts"
            :key="post.slug"
            :post="post"
          />
        </div>

        <a v-if="!loading && !loadError" class="archive-link" href="#/archive">
          查看全部文章
          <span aria-hidden="true">→</span>
        </a>
      </div>

      <aside class="sidebar">
        <section class="side-card">
          <span class="side-label">// PROFILE</span>
          <div class="side-profile">
            <span class="side-avatar" aria-hidden="true">❯_</span>
            <div>
              <strong>{{ profile.username }}</strong>
              <span>builder / writer</span>
            </div>
          </div>
          <p>后端工程师。喜欢把系统拆开、修好，再用文字记录整个过程。</p>
          <a class="text-link" href="#/about">
            cat about.md
            <span aria-hidden="true">↗</span>
          </a>
        </section>

        <section class="side-card">
          <span class="side-label">// NOW</span>
          <ul class="now-list">
            <li>
              <span>building</span>
              <strong>MyBlogProject</strong>
            </li>
            <li>
              <span>reading</span>
              <strong>DDIA</strong>
            </li>
          </ul>
        </section>
      </aside>
    </section>
  </div>
</template>

<style scoped>
.hero {
  --terminal-width: clamp(420px, 30vw, 460px);
  --hero-gap: clamp(40px, 5.5vw, 96px);
  position: relative;
  isolation: isolate;
  display: grid;
  grid-template-columns: minmax(0, 1fr) var(--terminal-width);
  align-items: center;
  gap: var(--hero-gap);
  min-height: calc(100vh - var(--header-height));
  min-height: calc(100svh - var(--header-height));
  padding-top: clamp(2rem, 5vh, 4rem);
  padding-bottom: clamp(4.5rem, 9vh, 7rem);
}

.hero::before {
  position: absolute;
  z-index: -1;
  top: 42%;
  left: calc(100% - var(--terminal-width) - (var(--hero-gap) / 2));
  width: clamp(520px, 65vw, 900px);
  height: clamp(420px, 50vw, 720px);
  background: var(--ambient-glow);
  content: '';
  filter: blur(18px);
  opacity: 0.66;
  pointer-events: none;
  transform: translate(-50%, -50%);
}

.hero-copy {
  animation: reveal-up 0.6s ease both;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 9px;
  padding: 7px 12px;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-soft);
  background:
    linear-gradient(120deg, var(--accent-soft), transparent 58%),
    var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.03em;
  backdrop-filter: blur(12px);
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 0 4px var(--accent-soft);
}

.hero h1 {
  max-width: 9.2em;
  margin-top: 1.5rem;
  font-size: clamp(2.3rem, 4.8vw, 3.8rem);
  font-weight: 650;
  letter-spacing: -0.055em;
  line-height: 1.06;
}

.hero-accent {
  color: var(--gradient-accent-a);
  background: linear-gradient(
    100deg,
    var(--gradient-accent-a) 0%,
    var(--gradient-accent-b) 52%,
    var(--gradient-accent-c) 100%
  );
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.hero-lead {
  max-width: 48ch;
  margin-top: 1.5rem;
  color: var(--text-soft);
  font-size: clamp(var(--text-lead), 1.4vw, 1.2rem);
  line-height: 1.85;
}

.hero-actions {
  display: flex;
  margin-top: 2rem;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.button {
  display: inline-flex;
  min-height: 3rem;
  align-items: center;
  justify-content: center;
  gap: 0.6rem;
  padding: 0.72rem 1.25rem;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  font-family: var(--font-sans);
  font-size: var(--text-meta);
  letter-spacing: 0.01em;
  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    border-color 0.2s ease,
    box-shadow 0.2s ease,
    transform 0.2s ease;
}

.button:hover {
  transform: translateY(-1px);
}

.button--primary {
  color: var(--accent-contrast);
  background: linear-gradient(120deg, var(--accent), var(--accent-hover));
  box-shadow: 0 12px 32px var(--accent-glow);
}

.button--primary:hover {
  box-shadow: 0 16px 42px var(--accent-glow);
}

.button--ghost {
  border-color: var(--border);
  color: var(--text-soft);
  background: var(--surface);
}

.button--ghost:hover {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.hero-stats {
  display: flex;
  margin-top: 3rem;
  gap: clamp(2rem, 5vw, 4rem);
}

.hero-stats div {
  display: flex;
  flex-direction: column-reverse;
  gap: 4px;
}

.hero-stats dt {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.hero-stats dd {
  margin: 0;
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 1.35rem;
  font-weight: 500;
  letter-spacing: -0.02em;
}

.hero-terminal {
  overflow: hidden;
  border: 1px solid transparent;
  border-radius: var(--radius-lg);
  background:
    linear-gradient(155deg, var(--surface-raised), var(--surface) 76%) padding-box,
    linear-gradient(145deg, var(--accent-line), var(--border) 42%, var(--accent-soft)) border-box;
  box-shadow: var(--shadow-lg);
  animation: reveal-up 0.65s 0.12s ease both;
}

.terminal-bar {
  display: flex;
  height: 42px;
  align-items: center;
  gap: 12px;
  padding: 0 16px;
  border-bottom: 1px solid var(--border);
  background: var(--surface-soft);
}

.terminal-dots {
  display: flex;
  gap: 6px;
}

.terminal-dots i {
  width: 9px;
  height: 9px;
  border-radius: 50%;
  background: var(--border-strong);
}

.terminal-dots i:first-child {
  background: var(--accent);
}

.terminal-title {
  margin-inline: auto;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.terminal-shell {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  opacity: 0.8;
}

.terminal-body {
  display: grid;
  gap: 1.4rem;
  padding: 1.7rem 1.6rem;
  font-family: var(--font-mono);
  font-size: var(--text-small);
  line-height: 1.72;
}

.terminal-block {
  min-width: 0;
}

.terminal-line {
  margin: 0;
  color: var(--text);
}

.terminal-line--caret {
  margin-top: 2px;
}

.prompt {
  margin-right: 8px;
  color: var(--accent);
}

.terminal-output {
  margin: 4px 0 0 1.1rem;
  overflow-wrap: anywhere;
  color: var(--text-soft);
}

.terminal-output--identity {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  column-gap: 0.35em;
  row-gap: 0.1em;
  font-size: var(--text-meta);
  letter-spacing: -0.01em;
}

.terminal-output--identity span {
  white-space: nowrap;
}

.terminal-log {
  display: grid;
  margin: 8px 0 0 1.1rem;
  gap: 0.35rem;
}

.terminal-log-line {
  display: flex;
  min-width: 0;
  align-items: baseline;
  gap: 0.65rem;
  color: var(--text-soft);
  font-size: var(--text-label);
  transition: color 0.2s ease;
}

.terminal-log-line:hover {
  color: var(--accent);
}

.terminal-log-hash {
  flex: 0 0 auto;
  color: var(--accent);
}

.terminal-log-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.terminal-log-empty {
  margin: 8px 0 0 1.1rem;
  color: var(--text-muted);
  font-size: var(--text-label);
}

.terminal-caret {
  display: inline-block;
  width: 8px;
  height: 15px;
  margin-bottom: -3px;
  background: var(--accent);
  animation: terminal-blink 1.05s step-end infinite;
}

.terminal-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.6rem 1rem;
  border-top: 1px solid var(--border);
  color: var(--text-muted);
  background: linear-gradient(180deg, transparent, var(--surface-soft));
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.03em;
}

.writing {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 21rem;
  align-items: start;
  gap: clamp(2rem, 5vw, 4.5rem);
  padding-top: clamp(2rem, 4vw, 3.5rem);
  scroll-margin-top: var(--header-height);
}

.section-head {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--border);
}

.section-kicker,
.side-label {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  font-weight: 500;
  letter-spacing: 0.06em;
}

.section-head h2 {
  margin-top: 0.5rem;
  font-size: clamp(1.6rem, 2.6vw, 2.1rem);
}

.search-box {
  display: flex;
  width: min(18rem, 100%);
  height: 2.75rem;
  align-items: center;
  gap: 9px;
  padding: 0 0.8rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  background: var(--surface);
  transition:
    border-color 0.2s ease,
    box-shadow 0.2s ease;
}

.search-box:focus-within {
  border-color: var(--accent-line);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.search-box input {
  min-width: 0;
  flex: 1;
  border: 0;
  outline: 0;
  color: var(--text);
  background: transparent;
  font-family: var(--font-mono);
  font-size: var(--text-meta);
}

.search-box input::placeholder {
  color: var(--text-muted);
}

.search-box input::-webkit-search-cancel-button {
  display: none;
}

.search-box kbd {
  display: inline-grid;
  min-width: 19px;
  height: 19px;
  place-items: center;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--surface-soft);
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1;
}

.search-clear {
  display: grid;
  width: 20px;
  height: 20px;
  flex: 0 0 auto;
  place-items: center;
  border-radius: 4px;
  color: var(--text-muted);
  background: transparent;
  font-size: 1rem;
  line-height: 1;
}

.search-clear:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.tag-filter {
  display: flex;
  margin-top: 18px;
  flex-wrap: wrap;
  gap: 7px;
}

.tag-filter button {
  padding: 0.35rem 0.7rem;
  border: 1px solid transparent;
  border-radius: 6px;
  color: var(--text-soft);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
  font-weight: 500;
  line-height: 1.6;
  transition:
    color 0.18s ease,
    background-color 0.18s ease,
    border-color 0.18s ease;
}

.tag-filter button:hover {
  color: var(--accent);
  border-color: var(--accent-line);
}

.tag-filter button.active {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.result-meta {
  display: flex;
  min-height: 2.5rem;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.result-meta button {
  color: var(--text-soft);
  background: transparent;
  font-family: var(--font-mono);
  font-size: var(--text-label);
  text-decoration: underline;
  text-underline-offset: 3px;
}

.result-meta button:hover {
  color: var(--accent);
}

.post-list {
  display: grid;
  gap: 10px;
}

.skeleton-card {
  display: flex;
  min-height: 168px;
  flex-direction: column;
  gap: 12px;
  padding: 24px;
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  background: var(--surface);
}

.skeleton-line {
  width: 88%;
  height: 10px;
  border-radius: 4px;
  background: var(--surface-soft);
  animation: skeleton-pulse 1.4s ease-in-out infinite;
}

.skeleton-line--short {
  width: 24%;
  height: 7px;
}

.skeleton-line--title {
  width: 66%;
  height: 20px;
  margin-top: 10px;
}

.skeleton-line--medium {
  width: 54%;
}

.empty-state {
  display: flex;
  min-height: 280px;
  flex-direction: column;
  align-items: flex-start;
  justify-content: center;
  padding: 32px;
  border: 1px dashed var(--border-strong);
  border-radius: var(--radius-lg);
  background: var(--surface);
}

.empty-command {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.empty-state h3 {
  margin-top: 16px;
  font-size: 1.35rem;
}

.empty-state p {
  margin-top: 8px;
  color: var(--text-soft);
  font-size: var(--text-small);
}

.empty-state .button {
  margin-top: 20px;
}

.sidebar {
  position: sticky;
  top: calc(var(--header-height) + 28px);
  display: grid;
  gap: 0.9rem;
}

.side-card {
  display: flex;
  flex-direction: column;
  gap: 0.9rem;
  padding: 1.35rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background:
    radial-gradient(circle at 100% 0, var(--surface-glow), transparent 48%),
    var(--surface);
}

.side-profile {
  display: flex;
  align-items: center;
  gap: 12px;
}

.side-avatar {
  display: grid;
  width: 2.75rem;
  height: 2.75rem;
  flex: 0 0 auto;
  place-items: center;
  border: 1px solid var(--accent-line);
  border-radius: 10px;
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
}

.side-profile div {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.side-profile strong {
  font-family: var(--font-mono);
  font-size: 1rem;
  font-weight: 600;
}

.side-profile span {
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.side-card > p {
  color: var(--text-soft);
  font-size: var(--text-small);
  line-height: 1.8;
}

.text-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.text-link:hover {
  color: var(--accent-hover);
}

.now-list {
  display: grid;
  gap: 10px;
  list-style: none;
}

.now-list li {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
  padding-bottom: 9px;
  border-bottom: 1px solid var(--border);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
}

.now-list li:last-child {
  padding-bottom: 0;
  border-bottom: 0;
}

.now-list span {
  color: var(--text-muted);
}

.now-list strong {
  color: var(--text-soft);
  font-weight: 500;
  text-align: right;
}

.archive-link {
  display: inline-flex;
  margin-top: 1.5rem;
  align-items: center;
  gap: 6px;
  color: var(--accent);
  font-size: var(--text-small);
}

.archive-link:hover {
  color: var(--accent-hover);
}

@keyframes reveal-up {
  from {
    opacity: 0;
    transform: translateY(16px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes terminal-blink {
  0%,
  48% {
    opacity: 1;
  }

  49%,
  100% {
    opacity: 0;
  }
}

@keyframes skeleton-pulse {
  0%,
  100% {
    opacity: 0.55;
  }

  50% {
    opacity: 1;
  }
}



@media (max-width: 960px) {
  .hero {
    gap: 42px;
    grid-template-columns: 1fr;
  }

  .hero::before {
    top: 46%;
    left: 50%;
  }

  .hero h1 {
    max-width: 10em;
  }

  .hero-terminal {
    width: 100%;
    max-width: none;
    justify-self: stretch;
  }

  .writing {
    grid-template-columns: 1fr;
  }

  .sidebar {
    position: static;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (min-width: 800px) and (max-width: 960px) {
  .terminal-body {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    column-gap: 36px;
    row-gap: 20px;
    padding: 26px;
  }

  .terminal-line--caret {
    grid-column: 1 / -1;
    margin-top: 0;
  }
}

@media (max-width: 720px) {
  .section-head {
    align-items: flex-start;
    flex-direction: column;
  }

  .search-box {
    width: 100%;
  }

  .sidebar {
    grid-template-columns: 1fr;
  }

  .hero-stats {
    margin-top: 2.25rem;
  }

  .hero::before {
    top: 50%;
  }

  .terminal-body {
    font-size: var(--text-small);
  }
}

@media (max-width: 480px) {
  .hero h1 {
    font-size: clamp(1.9rem, 9vw, 2.5rem);
  }

  .hero-actions .button {
    flex: 1;
  }

  .terminal-body {
    gap: 16px;
    padding: 20px;
    font-size: var(--text-small);
  }

  .terminal-output,
  .terminal-log,
  .terminal-log-empty {
    margin-left: 12px;
  }

  .terminal-footer {
    font-size: var(--text-label);
  }
}
</style>
