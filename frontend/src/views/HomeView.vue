<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import PostCard from '@/components/PostCard.vue'
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
  document.getElementById('writing')?.scrollIntoView({ behavior: 'smooth' })
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
          <div>
            <dt>CURIOSITY</dt>
            <dd>∞</dd>
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
          <p class="terminal-line"><span class="prompt">❯</span> whoami</p>
          <p class="terminal-output">sudo4 — backend engineer / writer</p>

          <p class="terminal-line"><span class="prompt">❯</span> cat now.txt</p>
          <p class="terminal-output">building a blog with Rust &amp; Vue</p>

          <p class="terminal-line"><span class="prompt">❯</span> ls ./stack</p>
          <div class="terminal-chips">
            <span>rust</span>
            <span>vue</span>
            <span>linux</span>
            <span>postgres</span>
          </div>

          <p class="terminal-line">
            <span class="prompt">❯</span>
            <span class="terminal-caret" aria-hidden="true"></span>
          </p>
        </div>

        <div class="terminal-footer">
          <span>UTF-8</span>
          <span>git:main</span>
          <span class="terminal-connected">
            <i aria-hidden="true"></i>
            connected
          </span>
        </div>
      </aside>
    </section>

    <section id="writing" class="writing container">
      <div class="writing-main">
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
            v-for="(post, index) in filteredPosts"
            :key="post.slug"
            :post="post"
            :index="index"
            :featured="index === 0 && !hasFilters"
          />
        </div>
      </div>

      <aside class="sidebar">
        <section class="side-card">
          <span class="side-label">// PROFILE</span>
          <div class="side-profile">
            <span class="side-avatar" aria-hidden="true">❯_</span>
            <div>
              <strong>sudo4</strong>
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
            <li>
              <span>status</span>
              <strong class="side-online">
                <i aria-hidden="true"></i>
                online
              </strong>
            </li>
          </ul>
        </section>

        <a class="side-card side-card--link" href="#/archive">
          <span class="side-label">// ARCHIVE</span>
          <span class="side-link-title">
            全部文章
            <span aria-hidden="true">↗</span>
          </span>
          <span class="side-link-meta">{{ postCountLabel }} entries, newest first</span>
        </a>
      </aside>
    </section>
  </div>
</template>

<style scoped>
.home {
  padding-top: clamp(48px, 8vw, 92px);
}

.hero {
  display: grid;
  grid-template-columns: minmax(0, 1.15fr) minmax(360px, 0.85fr);
  align-items: center;
  gap: clamp(40px, 7vw, 88px);
  padding-bottom: clamp(64px, 9vw, 108px);
}

.hero-copy {
  animation: reveal-up 0.6s ease both;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 9px;
  padding: 6px 10px;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-muted);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: 0.65rem;
  letter-spacing: 0.04em;
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
  margin-top: 24px;
  font-size: clamp(2.5rem, 5.4vw, 4.35rem);
  font-weight: 650;
  letter-spacing: -0.055em;
  line-height: 1.08;
}

.hero-accent {
  color: var(--accent);
}

.hero-lead {
  max-width: 52ch;
  margin-top: 24px;
  color: var(--text-muted);
  font-size: clamp(0.95rem, 1.3vw, 1.05rem);
  line-height: 1.85;
}

.hero-actions {
  display: flex;
  margin-top: 32px;
  flex-wrap: wrap;
  gap: 12px;
}

.button {
  display: inline-flex;
  min-height: 44px;
  align-items: center;
  justify-content: center;
  gap: 9px;
  padding: 10px 18px;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: 0.76rem;
  letter-spacing: 0.01em;
  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    border-color 0.2s ease,
    transform 0.2s ease;
}

.button:hover {
  transform: translateY(-1px);
}

.button--primary {
  color: var(--accent-contrast);
  background: var(--accent);
  box-shadow: 0 10px 24px var(--accent-glow);
}

.button--primary:hover {
  background: var(--accent-hover);
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
  margin-top: 46px;
  gap: clamp(28px, 5vw, 58px);
}

.hero-stats div {
  display: flex;
  flex-direction: column-reverse;
  gap: 4px;
}

.hero-stats dt {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.6rem;
  letter-spacing: 0.1em;
}

.hero-stats dd {
  margin: 0;
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 1.15rem;
  font-weight: 500;
  letter-spacing: -0.02em;
}

.hero-terminal {
  overflow: hidden;
  border: 1px solid var(--border-strong);
  border-radius: var(--radius-lg);
  background: var(--surface-raised);
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
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.64rem;
}

.terminal-shell {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.58rem;
  opacity: 0.7;
}

.terminal-body {
  min-height: 280px;
  padding: 24px;
  font-family: var(--font-mono);
  font-size: 0.78rem;
  line-height: 1.7;
}

.terminal-line {
  margin-top: 10px;
  color: var(--text);
}

.terminal-line:first-child {
  margin-top: 0;
}

.prompt {
  margin-right: 8px;
  color: var(--accent);
}

.terminal-output {
  margin: 4px 0 14px 18px;
  color: var(--text-muted);
}

.terminal-chips {
  display: flex;
  margin: 7px 0 14px 18px;
  flex-wrap: wrap;
  gap: 6px;
}

.terminal-chips span {
  padding: 2px 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--accent);
  background: var(--accent-soft);
  font-size: 0.68rem;
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
  padding: 9px 16px;
  border-top: 1px solid var(--border);
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: 0.58rem;
  letter-spacing: 0.04em;
}

.terminal-connected {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  color: var(--accent);
}

.terminal-connected i {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--accent);
}

.writing {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 292px;
  align-items: start;
  gap: clamp(32px, 5vw, 64px);
  scroll-margin-top: calc(var(--header-height) + 28px);
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
  font-size: 0.64rem;
  letter-spacing: 0.08em;
}

.section-head h2 {
  margin-top: 8px;
  font-size: clamp(1.65rem, 3vw, 2.05rem);
}

.search-box {
  display: flex;
  width: min(245px, 100%);
  height: 40px;
  align-items: center;
  gap: 9px;
  padding: 0 11px;
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
  font-size: 0.72rem;
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
  font-size: 0.62rem;
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
  padding: 4px 9px;
  border: 1px solid transparent;
  border-radius: 5px;
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: 0.66rem;
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
  min-height: 38px;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.64rem;
}

.result-meta button {
  color: var(--text-muted);
  background: transparent;
  font-family: var(--font-mono);
  font-size: 0.64rem;
  text-decoration: underline;
  text-underline-offset: 3px;
}

.result-meta button:hover {
  color: var(--accent);
}

.post-list {
  display: grid;
  gap: 14px;
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
  font-size: 0.7rem;
}

.empty-state h3 {
  margin-top: 16px;
  font-size: 1.2rem;
}

.empty-state p {
  margin-top: 8px;
  color: var(--text-muted);
  font-size: 0.88rem;
}

.empty-state .button {
  margin-top: 20px;
}

.sidebar {
  position: sticky;
  top: calc(var(--header-height) + 28px);
  display: grid;
  gap: 14px;
}

.side-card {
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 20px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.side-profile {
  display: flex;
  align-items: center;
  gap: 12px;
}

.side-avatar {
  display: grid;
  width: 42px;
  height: 42px;
  flex: 0 0 auto;
  place-items: center;
  border: 1px solid var(--accent-line);
  border-radius: 10px;
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: 0.78rem;
}

.side-profile div {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.side-profile strong {
  font-family: var(--font-mono);
  font-size: 0.85rem;
  font-weight: 600;
}

.side-profile span {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.62rem;
}

.side-card > p {
  color: var(--text-muted);
  font-size: 0.82rem;
  line-height: 1.75;
}

.text-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: 0.68rem;
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
  font-size: 0.65rem;
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

.side-online {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  color: var(--accent) !important;
}

.side-online i {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--accent);
}

.side-card--link {
  transition:
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.side-card--link:hover {
  border-color: var(--accent-line);
  background: var(--accent-soft);
}

.side-link-title {
  display: flex;
  align-items: center;
  justify-content: space-between;
  color: var(--text);
  font-size: 1.05rem;
  font-weight: 600;
  letter-spacing: -0.02em;
}

.side-card--link:hover .side-link-title {
  color: var(--accent);
}

.side-link-meta {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: 0.6rem;
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
    grid-template-columns: 1fr;
    gap: 48px;
  }

  .hero h1 {
    max-width: 10em;
  }

  .hero-terminal {
    max-width: 600px;
  }

  .writing {
    grid-template-columns: 1fr;
  }

  .sidebar {
    position: static;
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}

@media (max-width: 720px) {
  .home {
    padding-top: 40px;
  }

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
    margin-top: 36px;
  }

  .terminal-body {
    min-height: 250px;
  }
}

@media (max-width: 480px) {
  .hero h1 {
    font-size: clamp(2.1rem, 11vw, 2.8rem);
  }

  .hero-actions .button {
    flex: 1;
  }

  .terminal-body {
    padding: 20px;
  }

  .terminal-footer {
    font-size: 0.54rem;
  }
}
</style>
