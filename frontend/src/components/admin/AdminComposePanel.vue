<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import {
  createAdminPost,
  updateAdminPost,
  type AdminPost,
  type AdminPostPayload,
} from '@/services/admin'
import AppLink from '@/components/AppLink.vue'
import { clearEditingPost, useAdminPostStore } from '@/stores/adminPost'
import type { PostStatus } from '@/types/blog'
import { renderMarkdown } from '@/utils/markdown'

const { editingPost } = useAdminPostStore()

const title = ref('')
const slug = ref('')
const description = ref('')
const tags = ref('')
const publishedAt = ref(new Date().toISOString().slice(0, 10))
const readingMinutes = ref<number | ''>('')
const featured = ref(false)
const status = ref<PostStatus>('published')
const content = ref('')

const editingSlug = ref<string | null>(null)
const saving = ref(false)
const message = ref<{ type: 'ok' | 'error'; text: string } | null>(null)

const copyState = ref<'idle' | 'copied' | 'failed'>('idle')
let copyTimer: ReturnType<typeof setTimeout> | undefined

const mobileView = ref<'edit' | 'preview'>('edit')

const tagList = computed(() =>
  tags.value
    .split(',')
    .map((tag) => tag.trim())
    .filter(Boolean),
)

const previewHtml = computed(() => renderMarkdown(content.value))

const contentLength = computed(() => content.value.length)

// 编辑器 → 预览滚动同步（参考项目 EditorPage 的做法）
const editorRef = ref<HTMLTextAreaElement | null>(null)
const previewRef = ref<HTMLElement | null>(null)
const syncingScroll = ref(false)

function onEditorScroll() {
  if (syncingScroll.value) return

  const editor = editorRef.value
  const preview = previewRef.value

  if (!editor || !preview) return

  const ratio =
    editor.scrollTop / Math.max(1, editor.scrollHeight - editor.clientHeight)

  syncingScroll.value = true
  preview.scrollTop = ratio * (preview.scrollHeight - preview.clientHeight)
  requestAnimationFrame(() => {
    syncingScroll.value = false
  })
}

function insertSyntax(before: string, after = '', placeholder = '') {
  const el = editorRef.value

  if (!el) return

  const start = el.selectionStart
  const end = el.selectionEnd
  const selected = content.value.slice(start, end) || placeholder
  const next =
    content.value.slice(0, start) + before + selected + after + content.value.slice(end)

  content.value = next
  requestAnimationFrame(() => {
    el.focus()
    el.setSelectionRange(start + before.length, start + before.length + selected.length)
  })
}

const boldIcon = '<path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"/>'
const italicIcon =
  '<path d="M19 4h-9M14 20H5M15 4 9 20"/>'
const headingIcon = '<path d="M6 4v16M18 4v16M6 12h12"/>'
const quoteIcon =
  '<path d="M10 11H6a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h3a1 1 0 0 1 1 1v7a3 3 0 0 1-3 3"/><path d="M19 11h-4a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h3a1 1 0 0 1 1 1v7a3 3 0 0 1-3 3"/>'
const listIcon =
  '<path d="M8 6h13M8 12h13M8 18h13"/><path d="M3.5 6h.01M3.5 12h.01M3.5 18h.01"/>'
const codeIcon = '<path d="m16 18 6-6-6-6"/><path d="m8 6-6 6 6 6"/>'
const linkIcon =
  '<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>'

const toolbar = [
  { label: '加粗', icon: boldIcon, action: () => insertSyntax('**', '**', '加粗文字') },
  { label: '斜体', icon: italicIcon, action: () => insertSyntax('*', '*', '斜体文字') },
  { label: '标题', icon: headingIcon, action: () => insertSyntax('\n## ', '', '小节标题') },
  { label: '引用', icon: quoteIcon, action: () => insertSyntax('\n> ', '', '引用内容') },
  { label: '列表', icon: listIcon, action: () => insertSyntax('\n- ', '', '列表项') },
  { label: '代码', icon: codeIcon, action: () => insertSyntax('\n```\n', '\n```', 'code') },
  { label: '链接', icon: linkIcon, action: () => insertSyntax('[', '](https://)', '链接文字') },
]

const eyeIcon =
  '<path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/>'

const copyLabel = computed(() => {
  if (copyState.value === 'copied') return '已复制'
  if (copyState.value === 'failed') return '复制失败'
  return '复制 Markdown'
})

const saveLabel = computed(() => {
  if (saving.value) return '保存中…'
  if (editingSlug.value) return '保存并发布'
  return '发布文章'
})

function today(): string {
  return new Date().toISOString().slice(0, 10)
}

function fillForm(post: AdminPost) {
  editingSlug.value = post.slug
  title.value = post.title
  slug.value = post.slug
  description.value = post.description
  tags.value = post.tags.join(', ')
  publishedAt.value = post.publishedAt
  readingMinutes.value = post.readingMinutes
  featured.value = Boolean(post.featured)
  content.value = post.content
}

function resetForm() {
  editingSlug.value = null
  title.value = ''
  slug.value = ''
  description.value = ''
  tags.value = ''
  publishedAt.value = today()
  readingMinutes.value = ''
  featured.value = false
  status.value = 'published'
  content.value = ''
  message.value = null
  clearEditingPost()
}

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
    `status: ${status.value}`,
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
  anchor.download = `${(slug.value.trim() || 'untitled').replace(/[\\/:*?"<>|]+/g, '-')}.md`
  anchor.click()
  URL.revokeObjectURL(url)
}

function buildPayload(): AdminPostPayload {
  return {
    slug: slug.value.trim(),
    title: title.value.trim(),
    description: description.value.trim(),
    publishedAt: publishedAt.value,
    updatedAt: editingSlug.value ? today() : undefined,
    tags: tagList.value,
    readingMinutes: Number(readingMinutes.value) || 0,
    featured: featured.value,
    status: status.value,
    content: content.value,
  }
}

// 状态由操作自动判断：点击「发布」即为已发布，点击「存为草稿」即为草稿。
async function savePost(nextStatus: PostStatus = 'published') {
  status.value = nextStatus
  saving.value = true
  message.value = null

  try {
    const payload = buildPayload()
    const saved = editingSlug.value
      ? await updateAdminPost(editingSlug.value, payload)
      : await createAdminPost(payload)

    editingSlug.value = saved.slug
    slug.value = saved.slug
    message.value = {
      type: 'ok',
      text: saved.status === 'published' ? '已发布' : '已保存为草稿',
    }
    clearEditingPost()
  } catch (error) {
    message.value = {
      type: 'error',
      text: error instanceof Error ? error.message : '保存失败',
    }
  } finally {
    saving.value = false
  }
}

async function saveAsDraft() {
  await savePost('draft')
}

onMounted(() => {
  const post = editingPost.value

  if (post) {
    fillForm({ ...post, tags: [...post.tags] })
  }
})

// 发布页不做内存缓存：离开页面即释放编辑状态，下次进入始终是干净的新建表单。
onBeforeUnmount(() => {
  clearEditingPost()
  if (copyTimer) clearTimeout(copyTimer)
})

// 预览内容变化后，保持滚动比例大致对齐。
watch(previewHtml, async () => {
  await nextTick()

  const editor = editorRef.value
  const preview = previewRef.value

  if (!editor || !preview) return

  const ratio =
    editor.scrollTop / Math.max(1, editor.scrollHeight - editor.clientHeight)
  preview.scrollTop = ratio * (preview.scrollHeight - preview.clientHeight)
})
</script>

<template>
  <div class="compose-wrap">
    <!-- 元信息 -->
    <section class="compose-card meta-card" aria-label="文章信息">
      <input
        v-model="title"
        class="meta-title"
        type="text"
        placeholder="文章标题"
        maxlength="80"
      />

      <div class="meta-grid">
        <label class="field">
          <span>Slug</span>
          <input v-model="slug" type="text" placeholder="terminal-workflow" spellcheck="false" />
        </label>

        <label class="field">
          <span>标签（逗号分隔）</span>
          <input v-model="tags" type="text" placeholder="Rust, Backend" />
        </label>

        <label class="field field--wide">
          <span>摘要</span>
          <textarea
            v-model="description"
            rows="2"
            placeholder="一句话说明这篇文章（可选）"
          ></textarea>
        </label>

        <label class="field">
          <span>阅读时长（分钟，留空自动估算）</span>
          <input v-model.number="readingMinutes" type="number" min="0" step="1" placeholder="自动" />
        </label>

        <label class="check-field">
          <input v-model="featured" type="checkbox" />
          <span>设为首页精选文章</span>
        </label>
      </div>
    </section>

    <!-- 移动端：编辑 / 预览切换 -->
    <div class="compose-switch" role="tablist" aria-label="编辑或预览">
      <button
        class="switch-option"
        :class="{ active: mobileView === 'edit' }"
        type="button"
        role="tab"
        :aria-selected="mobileView === 'edit'"
        @click="mobileView = 'edit'"
      >
        编辑
      </button>
      <button
        class="switch-option"
        :class="{ active: mobileView === 'preview' }"
        type="button"
        role="tab"
        :aria-selected="mobileView === 'preview'"
        @click="mobileView = 'preview'"
      >
        <svg
          viewBox="0 0 24 24"
          width="14"
          height="14"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
          v-html="eyeIcon"
        ></svg>
        预览
      </button>
    </div>

    <!-- 编辑 + 预览 -->
    <div class="compose-grid">
      <section
        class="compose-card pane editor-card"
        :class="{ 'pane--hidden': mobileView !== 'edit' }"
        aria-label="Markdown 编辑器"
      >
        <div class="editor-toolbar">
          <button
            v-for="item in toolbar"
            :key="item.label"
            class="tool-button"
            type="button"
            :title="item.label"
            :aria-label="item.label"
            @click="item.action"
          >
            <svg
              viewBox="0 0 24 24"
              width="15"
              height="15"
              fill="none"
              stroke="currentColor"
              stroke-width="1.9"
              stroke-linecap="round"
              stroke-linejoin="round"
              aria-hidden="true"
              v-html="item.icon"
            ></svg>
          </button>

          <span class="editor-count">{{ contentLength }} 字</span>
        </div>

        <textarea
          ref="editorRef"
          v-model="content"
          class="editor-textarea"
          placeholder="用 Markdown 开始写作……"
          spellcheck="false"
          @scroll.passive="onEditorScroll"
        ></textarea>
      </section>

      <section
        class="compose-card pane preview-card"
        :class="{ 'pane--hidden': mobileView !== 'preview' }"
        aria-label="Markdown 预览"
      >
        <div class="preview-heading">
          <svg
            viewBox="0 0 24 24"
            width="14"
            height="14"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            v-html="eyeIcon"
          ></svg>
          实时预览
        </div>

        <div ref="previewRef" class="preview-body">
          <h2 class="preview-title">{{ title || '未命名文章' }}</h2>

          <ul v-if="tagList.length" class="preview-tags">
            <li v-for="tag in tagList" :key="tag">{{ tag }}</li>
          </ul>

          <div v-if="previewHtml" class="preview-content" v-html="previewHtml"></div>

          <div v-else class="preview-empty">
            <p>还没有内容</p>
            <span>在编辑区输入 Markdown，这里会实时显示渲染结果。</span>
          </div>
        </div>
      </section>
    </div>

    <!-- 操作条 -->
    <div class="compose-actions">
      <p v-if="message" class="form-message" :class="`form-message--${message.type}`">
        {{ message.text }}
        <template v-if="message.type === 'ok'">
          ·
          <AppLink class="message-link" href="/admin/posts">返回文章管理</AppLink>
        </template>
      </p>

      <div class="action-row action-row--primary">
        <button
          class="action action--publish"
          type="button"
          :disabled="saving || !slug.trim() || !title.trim()"
          @click="savePost('published')"
        >
          {{ saveLabel }}
        </button>
        <button
          class="action"
          type="button"
          :disabled="saving || !slug.trim() || !title.trim()"
          @click="saveAsDraft"
        >
          存为草稿
        </button>
      </div>

      <div class="action-row action-row--secondary">
        <button class="action action--small" type="button" @click="resetForm">
          {{ editingSlug ? '放弃修改' : '清空重写' }}
        </button>
        <button class="action action--small" type="button" @click="copyMarkdown">
          {{ copyLabel }}
        </button>
        <button class="action action--small" type="button" @click="downloadMarkdown">
          下载 .md
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.compose-wrap {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.compose-card {
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  background: var(--surface);
}

/* ---- 元信息 ---- */

.meta-card {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.25rem;
}

.meta-title {
  width: 100%;
  padding: 0.65rem 1rem;
  border: 0;
  border-radius: var(--radius-md);
  outline: 0;
  color: var(--text);
  background: var(--surface-soft);
  font-size: 1.15rem;
  font-weight: 650;
  letter-spacing: -0.02em;
  transition: box-shadow 0.2s ease;
}

.meta-title::placeholder {
  color: var(--text-muted);
  font-weight: 500;
}

.meta-title:focus {
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.meta-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.9rem;
}

.field {
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: 0.4rem;
}

.field--wide {
  grid-column: 1 / -1;
}

.field > span {
  color: var(--text-muted);
  font-size: var(--text-label);
}

.field input,
.field select,
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

.field input,
.field select {
  height: 2.5rem;
  padding: 0 0.7rem;
}

.field input[type='date'] {
  color-scheme: light;
}

[data-theme='dark'] .field input[type='date'] {
  color-scheme: dark;
}

.field textarea {
  padding: 0.6rem 0.7rem;
  resize: vertical;
}

.field input:focus,
.field select:focus,
.field textarea:focus {
  border-color: var(--accent-line);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.check-field {
  display: inline-flex;
  min-height: 2.5rem;
  align-items: center;
  align-self: end;
  gap: 0.5rem;
  color: var(--text-soft);
  font-size: var(--text-label);
}

.check-field input {
  width: 1rem;
  height: 1rem;
  accent-color: var(--accent);
}

/* ---- 移动端切换 ---- */

.compose-switch {
  display: flex;
  gap: 0.25rem;
  padding: 0.25rem;
  border-radius: 999px;
  background: var(--surface-soft);
}

.switch-option {
  display: flex;
  flex: 1;
  align-items: center;
  justify-content: center;
  gap: 0.4rem;
  height: 2.5rem;
  border-radius: 999px;
  color: var(--text-muted);
  background: transparent;
  font-size: var(--text-small);
  font-weight: 500;
  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    box-shadow 0.2s ease;
}

.switch-option.active {
  color: var(--text);
  background: var(--surface);
  box-shadow: var(--shadow-sm);
}

/* ---- 编辑 + 预览 ---- */

.compose-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  align-items: stretch;
  gap: 1.25rem;
}

.pane {
  display: flex;
  min-width: 0;
  height: clamp(26rem, calc(100dvh - 30rem), 46rem);
  flex-direction: column;
  overflow: hidden;
}

.editor-toolbar {
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  gap: 0.15rem;
  padding: 0.5rem 0.75rem;
  border-bottom: 1px solid var(--border);
}

.tool-button {
  display: grid;
  width: 2.1rem;
  height: 2.1rem;
  flex: 0 0 auto;
  place-items: center;
  border: 0;
  border-radius: 10px;
  color: var(--text-muted);
  background: transparent;
  transition:
    color 0.2s ease,
    background-color 0.2s ease;
}

.tool-button:hover {
  color: var(--text);
  background: var(--surface-soft);
}

.editor-count {
  margin-left: auto;
  padding-right: 0.35rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.editor-textarea {
  flex: 1;
  min-height: 0;
  width: 100%;
  padding: 1rem 1.25rem;
  border: 0;
  outline: 0;
  resize: none;
  color: var(--text);
  background: transparent;
  font-family: var(--font-mono);
  font-size: 0.85rem;
  line-height: 1.75;
}

.editor-textarea::placeholder {
  color: var(--text-muted);
}

/* ---- 预览 ---- */

.preview-heading {
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  gap: 0.4rem;
  padding: 0.7rem 1.25rem;
  border-bottom: 1px solid var(--border);
  background: var(--surface-soft);
  color: var(--text-muted);
  font-size: var(--text-label);
  font-weight: 500;
}

.preview-body {
  flex: 1;
  min-height: 0;
  padding: 1.25rem 1.5rem 1.75rem;
  overflow-y: auto;
}

.preview-title {
  color: var(--text);
  font-size: clamp(1.35rem, 2.6vw, 1.7rem);
  letter-spacing: -0.035em;
  line-height: 1.25;
}

.preview-tags {
  display: flex;
  margin-top: 0.75rem;
  flex-wrap: wrap;
  gap: 0.35rem;
  list-style: none;
}

.preview-tags li {
  padding: 0.15rem 0.55rem;
  border-radius: 999px;
  color: var(--accent);
  background: var(--accent-soft);
  font-size: 0.72rem;
  font-weight: 500;
}

.preview-content {
  margin-top: 1.25rem;
  padding-top: 1.25rem;
  border-top: 1px solid var(--border);
  color: var(--text-soft);
  font-size: var(--text-body);
  line-height: 1.85;
}

.preview-content :deep(h1),
.preview-content :deep(h2),
.preview-content :deep(h3),
.preview-content :deep(h4) {
  margin: 1.6em 0 0.6em;
  color: var(--text);
  font-weight: 650;
  letter-spacing: -0.02em;
  line-height: 1.35;
}

.preview-content :deep(h1) {
  font-size: 1.4rem;
}

.preview-content :deep(h2) {
  font-size: 1.2rem;
}

.preview-content :deep(h3) {
  font-size: 1.05rem;
}

.preview-content :deep(p) {
  margin: 0.85em 0;
}

.preview-content :deep(ul),
.preview-content :deep(ol) {
  display: grid;
  margin: 0.85em 0;
  padding-left: 1.4em;
  gap: 0.25em;
}

.preview-content :deep(blockquote) {
  margin: 1.1em 0;
  padding: 0.7rem 1rem;
  border-left: 3px solid var(--accent);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  color: var(--text-soft);
  background: var(--accent-soft);
}

.preview-content :deep(code) {
  padding: 0.12em 0.36em;
  border-radius: 5px;
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: 0.86em;
}

.preview-content :deep(pre) {
  margin: 1.1em 0;
  overflow-x: auto;
  border-radius: var(--radius-md);
  background: var(--code-bg);
}

.preview-content :deep(pre code) {
  display: block;
  padding: 0.9rem 1.1rem;
  color: var(--code-text);
  background: transparent;
  font-size: 0.8rem;
  line-height: 1.7;
}

.preview-empty {
  display: flex;
  min-height: 12rem;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  margin-top: 1.25rem;
  padding: 1rem;
  border: 1px dashed var(--border-strong);
  border-radius: var(--radius-md);
  text-align: center;
}

.preview-empty p {
  color: var(--text-soft);
  font-size: var(--text-small);
  font-weight: 600;
}

.preview-empty span {
  max-width: 18rem;
  color: var(--text-muted);
  font-size: var(--text-label);
  line-height: 1.7;
}

/* ---- 操作条 ---- */

.compose-actions {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
}

.action-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.6rem;
}

.action-row--secondary {
  gap: 0.5rem;
}

.action {
  display: inline-flex;
  min-height: 2.6rem;
  align-items: center;
  justify-content: center;
  padding: 0.5rem 1.1rem;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-soft);
  background: var(--surface);
  font-size: var(--text-small);
  font-weight: 500;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.action--small {
  min-height: 2.25rem;
  padding: 0.35rem 0.85rem;
  color: var(--text-muted);
  font-size: var(--text-label);
}

.action:hover:not(:disabled) {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.action:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.action--publish {
  border-color: transparent;
  color: var(--accent-contrast);
  background: var(--accent);
}

.action--publish:hover:not(:disabled) {
  border-color: transparent;
  color: var(--accent-contrast);
  background: var(--accent-hover);
}

.form-message {
  margin: 0;
  padding: 0.6rem 0.8rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  font-size: var(--text-small);
}

.message-link {
  color: inherit;
  text-decoration: underline;
  text-underline-offset: 3px;
}

.form-message--ok {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.form-message--error {
  border-color: var(--danger-line);
  color: var(--danger);
  background: var(--danger-soft);
}

/* ---- 响应式 ---- */

@media (min-width: 981px) {
  .compose-switch {
    display: none;
  }
}

@media (max-width: 980px) {
  .compose-grid {
    grid-template-columns: 1fr;
  }

  .pane {
    height: auto;
  }

  .pane--hidden {
    display: none;
  }

  .editor-textarea {
    min-height: 22rem;
  }

  .meta-grid {
    grid-template-columns: 1fr;
  }

  /* 主操作：发布占更大比重，与存为草稿并排铺满 */
  .action-row--primary .action {
    flex: 1;
    min-height: 2.75rem;
  }

  .action-row--primary .action--publish {
    flex: 1.6;
  }

  /* 次要操作：小号按钮等宽铺满一行 */
  .action-row--secondary .action--small {
    flex: 1;
    padding-inline: 0.5rem;
    white-space: nowrap;
  }
}
</style>
