<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
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

const saveLabel = computed(() => {
  if (saving.value) return '保存中…'
  if (editingSlug.value) return '保存修改'
  return status.value === 'published' ? '发布文章' : '保存为草稿'
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
  status.value = post.status
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
  anchor.download = `${filename.value.replace(/[\\/:*?"<>|]+/g, '-')}.md`
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

async function savePost() {
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
  status.value = 'draft'
  await savePost()
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
</script>

<template>
  <div class="compose-wrap">
    <section class="admin-panel editor-panel">
      <div class="panel-heading editor-heading">
        <span>{{ editingSlug ? `EDIT · ${editingSlug}` : 'EDITOR · NEW' }}</span>
        <div class="heading-actions">
          <AppLink v-if="editingSlug" class="action action--secondary action--small" href="/admin/posts">
            ← 返回文章管理
          </AppLink>
          <AppLink v-else class="action action--secondary action--small" href="/admin/posts">
            文章管理
          </AppLink>
        </div>
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
          <span>发布状态</span>
          <select v-model="status">
            <option value="published">已发布</option>
            <option value="draft">草稿</option>
          </select>
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

      <label class="field field--full">
        <span>正文 Markdown</span>
        <textarea
          v-model="content"
          class="editor-textarea"
          placeholder="## 开始写正文&#10;&#10;支持标题、列表、引用和代码块。"
          spellcheck="false"
        ></textarea>
      </label>

      <p v-if="message" class="form-message" :class="`form-message--${message.type}`">
        {{ message.text }}
        <template v-if="message.type === 'ok'">
          ·
          <AppLink class="message-link" href="/admin/posts">返回文章管理</AppLink>
        </template>
      </p>

      <div class="form-actions">
        <button
          class="action action--publish"
          type="button"
          :disabled="saving || !slug.trim() || !title.trim()"
          @click="savePost"
        >
          {{ saveLabel }}
        </button>
        <button
          v-if="status !== 'draft'"
          class="action action--secondary"
          type="button"
          :disabled="saving || !slug.trim() || !title.trim()"
          @click="saveAsDraft"
        >
          存为草稿
        </button>
        <button class="action action--secondary" type="button" @click="resetForm">
          {{ editingSlug ? '放弃修改' : '清空重写' }}
        </button>
        <button class="action action--secondary" type="button" @click="copyMarkdown">
          {{ copyLabel }}
        </button>
        <button class="action action--secondary" type="button" @click="downloadMarkdown">
          下载 .md
        </button>
      </div>
    </section>

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
          <span>{{ status === 'published' ? 'PUBLISHED' : 'DRAFT' }}</span>
          <span>/</span>
          <span>{{ readingMinutes || 'AUTO' }} MIN</span>
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
</template>

<style scoped>
.compose-wrap {
  display: grid;
  grid-template-columns: minmax(0, 1.08fr) minmax(320px, 0.92fr);
  align-items: start;
  gap: clamp(1.25rem, 3vw, 2rem);
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

.heading-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
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
.field select:focus,
.field textarea:focus {
  border-color: var(--accent-line);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.editor-textarea {
  min-height: 26rem;
  resize: vertical;
  font-family: var(--font-mono) !important;
  font-size: var(--text-small) !important;
  line-height: 1.75;
}

.check-field {
  display: inline-flex;
  min-height: 2.6rem;
  align-items: center;
  align-self: end;
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

.action:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.action--secondary:hover:not(:disabled) {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.action--publish {
  border-color: transparent;
  color: var(--accent-contrast);
  background: var(--accent);
}

.action--small {
  min-height: 2rem;
  padding: 0.3rem 0.6rem;
}

.preview-panel {
  position: sticky;
  top: calc(var(--header-height) + 1.5rem);
  max-height: calc(100svh - var(--header-height) - 3rem);
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
  min-height: 14rem;
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
  .compose-wrap {
    grid-template-columns: 1fr;
  }

  .preview-panel {
    position: static;
    max-height: none;
  }
}

@media (max-width: 640px) {
  .field-grid {
    grid-template-columns: 1fr;
  }

  .editor-panel {
    padding: 1rem;
  }

  .form-actions .action {
    flex: 1;
  }
}
</style>
