<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from 'vue'
import { renderMarkdown } from '@/utils/markdown'

const title = ref('')
const slug = ref('')
const description = ref('')
const tags = ref('')
const publishedAt = ref(new Date().toISOString().slice(0, 10))
const readingMinutes = ref(6)
const featured = ref(false)
const content = ref('')
const copyState = ref<'idle' | 'copied' | 'failed'>('idle')
let copyTimer: ReturnType<typeof setTimeout> | undefined

const tagList = computed(() =>
  tags.value
    .split(',')
    .map((tag) => tag.trim())
    .filter(Boolean),
)

const filename = computed(() => slug.value.trim() || 'untitled')

const previewHtml = computed(() => renderMarkdown(content.value))

const copyLabel = computed(() => {
  if (copyState.value === 'copied') return '已复制'
  if (copyState.value === 'failed') return '复制失败'
  return '复制 Markdown'
})

function inlineValue(value: string): string {
  return value.replace(/\s+/g, ' ').trim()
}

function buildMarkdown(): string {
  const minutes = Math.max(1, Number(readingMinutes.value) || 1)
  const lines = [
    '---',
    `title: ${inlineValue(title.value)}`,
    `description: ${inlineValue(description.value)}`,
    `publishedAt: ${publishedAt.value}`,
    `tags: ${tagList.value.join(', ')}`,
    `readingMinutes: ${minutes}`,
    ...(featured.value ? ['featured: true'] : []),
    '---',
    '',
    content.value.trim(),
    '',
  ]

  return lines.join('\n')
}

async function copyMarkdown() {
  try {
    await navigator.clipboard.writeText(buildMarkdown())
    copyState.value = 'copied'
  } catch {
    copyState.value = 'failed'
  }

  if (copyTimer) clearTimeout(copyTimer)
  copyTimer = setTimeout(() => {
    copyState.value = 'idle'
  }, 2000)
}

function downloadMarkdown() {
  const blob = new Blob([buildMarkdown()], { type: 'text/markdown;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')

  anchor.href = url
  anchor.download = `${filename.value.replace(/[\\/:*?"<>|]+/g, '-')}.md`
  anchor.click()
  URL.revokeObjectURL(url)
}

onBeforeUnmount(() => {
  if (copyTimer) clearTimeout(copyTimer)
})
</script>

<template>
  <section class="admin-page page-with-glow container">
    <header class="admin-header">
      <span class="page-kicker">// ADMIN · PUBLISH</span>
      <h1>发布 Markdown</h1>
      <p>
        在浏览器里整理文章、检查 Frontmatter 和预览正文。认证与发布 API 接入后，这里会直接写入后台。
      </p>
    </header>

    <section class="auth-strip">
      <span class="auth-icon" aria-hidden="true">
        <svg
          viewBox="0 0 24 24"
          width="18"
          height="18"
          fill="none"
          stroke="currentColor"
          stroke-width="1.7"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect x="4" y="10" width="16" height="10" rx="2" />
          <path d="M8 10V7a4 4 0 0 1 8 0v3" />
        </svg>
      </span>
      <div class="auth-copy">
        <span>// AUTH REQUIRED</span>
        <strong>认证接口尚未接入</strong>
        <p>当前页面只用于撰写和预览，发布按钮保持禁用；认证必须在服务端完成。</p>
      </div>
      <span class="auth-state">offline</span>
    </section>

    <div class="admin-grid">
      <form class="admin-panel editor-panel" @submit.prevent>
        <div class="panel-heading">
          <span>// EDITOR</span>
          <span>{{ filename }}.md</span>
        </div>

        <div class="field-grid">
          <label class="field field--full">
            <span>标题</span>
            <input v-model="title" type="text" placeholder="例如：我的终端工作流" />
          </label>

          <label class="field">
            <span>文件名 / slug</span>
            <input v-model="slug" type="text" placeholder="terminal-workflow" spellcheck="false" />
          </label>

          <label class="field">
            <span>发布日期</span>
            <input v-model="publishedAt" type="date" />
          </label>

          <label class="field field--full">
            <span>摘要</span>
            <input v-model="description" type="text" placeholder="一句话说明这篇文章" />
          </label>

          <label class="field">
            <span>标签</span>
            <input v-model="tags" type="text" placeholder="Rust, Backend" />
          </label>

          <label class="field">
            <span>阅读时长（分钟）</span>
            <input v-model.number="readingMinutes" type="number" min="1" step="1" />
          </label>

          <label class="check-field field--full">
            <input v-model="featured" type="checkbox" />
            <span>设为首页精选文章</span>
          </label>
        </div>

        <label class="field field--full">
          <span>正文 Markdown</span>
          <textarea
            v-model="content"
            class="editor-textarea"
            placeholder="## 开始写正文&#10;&#10;支持标题、列表、引用和代码块。"
            spellcheck="false"
          ></textarea>
        </label>

        <div class="form-actions">
          <button class="action action--publish" type="button" disabled title="认证与发布 API 尚未接入">
            发布 · 待认证
          </button>
          <button class="action action--secondary" type="button" @click="copyMarkdown">
            {{ copyLabel }}
          </button>
          <button class="action action--secondary" type="button" @click="downloadMarkdown">
            下载 .md
          </button>
          <span class="action-note">认证通过后才会启用发布。</span>
        </div>
      </form>

      <aside class="admin-panel preview-panel" aria-label="Markdown 预览">
        <div class="panel-heading preview-heading">
          <span class="preview-dots" aria-hidden="true">
            <i></i>
            <i></i>
            <i></i>
          </span>
          <span>preview.md</span>
          <span>{{ filename }}.md</span>
        </div>

        <div class="preview-body">
          <div class="preview-meta">
            <span>{{ publishedAt || '未设置日期' }}</span>
            <span>/</span>
            <span>{{ readingMinutes || 0 }} MIN</span>
          </div>

          <h2 class="preview-title">{{ title || '未命名文章' }}</h2>
          <p v-if="description" class="preview-description">{{ description }}</p>

          <ul v-if="tagList.length" class="preview-tags">
            <li v-for="tag in tagList" :key="tag">#{{ tag }}</li>
          </ul>

          <div v-if="previewHtml" class="preview-content" v-html="previewHtml"></div>
          <div v-else class="preview-empty">
            <span>❯ cat preview.md</span>
            <p>在左侧输入 Markdown，这里会显示渲染结果。</p>
          </div>
        </div>
      </aside>
    </div>
  </section>
</template>

<style scoped>
.admin-page {
  padding-top: clamp(3rem, 6vw, 6rem);
}

.admin-header {
  max-width: 54rem;
}

.page-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  font-weight: 500;
  letter-spacing: 0.08em;
}

.admin-header h1 {
  margin-top: 0.75rem;
  font-size: clamp(2.4rem, 5vw, 4rem);
  letter-spacing: -0.055em;
}

.admin-header p {
  max-width: 52ch;
  margin-top: 1rem;
  color: var(--text-soft);
  font-size: var(--text-body);
  line-height: 1.8;
}

.auth-strip {
  display: flex;
  margin-top: 2rem;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.25rem;
  border: 1px solid var(--accent-line);
  border-radius: var(--radius-md);
  background:
    linear-gradient(120deg, var(--accent-soft), transparent 62%),
    var(--surface);
}

.auth-icon {
  display: grid;
  width: 2.5rem;
  height: 2.5rem;
  flex: 0 0 auto;
  place-items: center;
  border: 1px solid var(--accent-line);
  border-radius: 8px;
  color: var(--accent);
  background: var(--accent-soft);
}

.auth-copy {
  display: flex;
  min-width: 0;
  flex: 1;
  flex-direction: column;
  gap: 0.1rem;
}

.auth-copy > span {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.06em;
}

.auth-copy strong {
  font-size: var(--text-small);
  font-weight: 600;
}

.auth-copy p {
  color: var(--text-muted);
  font-size: var(--text-small);
}

.auth-state {
  padding: 0.25rem 0.5rem;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.admin-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.08fr) minmax(320px, 0.92fr);
  align-items: start;
  gap: clamp(1.5rem, 4vw, 3rem);
  margin-top: clamp(1.5rem, 4vw, 2.5rem);
}

.admin-panel {
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.editor-panel {
  display: grid;
  gap: 1.25rem;
  padding: 1.35rem;
}

.panel-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.04em;
}

.field-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1rem;
}

.field {
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: 0.45rem;
}

.field--full {
  grid-column: 1 / -1;
}

.field > span {
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.field input,
.field textarea {
  width: 100%;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  outline: 0;
  color: var(--text);
  background: var(--surface-soft);
  font-family: var(--font-sans);
  font-size: var(--text-small);
  transition:
    border-color 0.2s ease,
    box-shadow 0.2s ease;
}

.field input {
  height: 2.6rem;
  padding: 0 0.7rem;
}

.field input[type='date'] {
  color-scheme: light;
}

[data-theme='dark'] .field input[type='date'] {
  color-scheme: dark;
}

.field textarea {
  padding: 0.8rem;
}

.field input:focus,
.field textarea:focus {
  border-color: var(--accent-line);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.editor-textarea {
  min-height: 28rem;
  resize: vertical;
  font-family: var(--font-mono) !important;
  font-size: var(--text-small) !important;
  line-height: 1.75;
}

.check-field {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.check-field input {
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent);
}

.form-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.75rem;
  padding-top: 1.25rem;
  border-top: 1px solid var(--border);
}

.action {
  display: inline-flex;
  min-height: 2.65rem;
  align-items: center;
  justify-content: center;
  padding: 0.55rem 0.9rem;
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

.action--publish {
  border-color: transparent;
  color: var(--accent-contrast);
  background: var(--accent);
}

.action--publish:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.action-note {
  color: var(--text-muted);
  font-size: var(--text-label);
}

.preview-panel {
  position: sticky;
  top: calc(var(--header-height) + 1.75rem);
  max-height: calc(100svh - var(--header-height) - 3.5rem);
  overflow-y: auto;
}

.preview-heading {
  position: sticky;
  z-index: 1;
  top: 0;
  padding: 0.7rem 1rem;
  border-bottom: 1px solid var(--border);
  background: var(--surface-soft);
}

.preview-dots {
  display: flex;
  gap: 5px;
}

.preview-dots i {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--border-strong);
}

.preview-dots i:first-child {
  background: var(--accent);
}

.preview-body {
  padding: clamp(1.25rem, 3vw, 2rem);
}

.preview-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.preview-title {
  margin-top: 1rem;
  color: var(--text);
  font-size: clamp(1.6rem, 3vw, 2.1rem);
  letter-spacing: -0.04em;
  line-height: 1.2;
}

.preview-description {
  margin-top: 0.65rem;
  color: var(--text-soft);
  font-size: var(--text-small);
  line-height: 1.8;
}

.preview-tags {
  display: flex;
  margin-top: 1rem;
  flex-wrap: wrap;
  gap: 0.4rem;
  list-style: none;
}

.preview-tags li {
  padding: 0.2rem 0.45rem;
  border-radius: 4px;
  color: var(--text-soft);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.preview-content {
  margin-top: 1.5rem;
  color: var(--text-soft);
  font-size: var(--text-body);
  line-height: 1.85;
}

.preview-content :deep(h2) {
  margin: 2em 0 0.65em;
  color: var(--text);
  font-size: 1.3rem;
}

.preview-content :deep(h3) {
  margin: 1.6em 0 0.55em;
  color: var(--text);
  font-size: 1.1rem;
}

.preview-content :deep(p) {
  margin: 0.9em 0;
}

.preview-content :deep(ul),
.preview-content :deep(ol) {
  display: grid;
  margin: 0.9em 0;
  padding-left: 1.4em;
  gap: 0.3em;
}

.preview-content :deep(blockquote) {
  margin: 1.2em 0;
  padding: 0.8rem 1rem;
  border-left: 2px solid var(--accent);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  color: var(--text);
  background: var(--accent-soft);
}

.preview-content :deep(code) {
  padding: 0.12em 0.36em;
  border-radius: 4px;
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: 0.86em;
}

.preview-content :deep(pre) {
  margin: 1.2em 0;
  overflow-x: auto;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  background: var(--code-bg);
}

.preview-content :deep(pre code) {
  display: block;
  padding: 0.9rem 1rem;
  color: var(--code-text);
  background: transparent;
}

.preview-empty {
  display: flex;
  min-height: 16rem;
  flex-direction: column;
  justify-content: center;
  gap: 0.6rem;
  margin-top: 1.5rem;
  padding: 1rem;
  border: 1px dashed var(--border-strong);
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.preview-empty p {
  font-family: var(--font-sans);
  font-size: var(--text-small);
}

@media (max-width: 980px) {
  .admin-grid {
    grid-template-columns: 1fr;
  }

  .preview-panel {
    position: static;
    max-height: none;
  }
}

@media (max-width: 640px) {
  .auth-strip {
    align-items: flex-start;
  }

  .auth-state {
    display: none;
  }

  .field-grid {
    grid-template-columns: 1fr;
  }

  .editor-panel {
    padding: 1rem;
  }

  .form-actions .action {
    flex: 1;
  }

  .action-note {
    width: 100%;
  }
}
</style>
