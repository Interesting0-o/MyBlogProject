<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { createPostComment, fetchPostComments } from '@/services/comments'
import type { Comment } from '@/types/blog'

const props = defineProps<{
  slug: string
}>()

const comments = ref<Comment[]>([])
const loading = ref(true)
const loadError = ref(false)

const author = ref('')
const content = ref('')
const submitting = ref(false)
const submitState = ref<'idle' | 'pending' | 'done' | 'error'>('idle')
const submitError = ref<string | null>(null)

const count = computed(() => comments.value.length)

function formatTime(iso: string): string {
  return iso.replace('T', ' ').replace('Z', ' UTC')
}

async function loadComments() {
  loading.value = true
  loadError.value = false

  try {
    comments.value = await fetchPostComments(props.slug)
  } catch {
    loadError.value = true
  } finally {
    loading.value = false
  }
}

async function submit() {
  if (!author.value.trim() || !content.value.trim() || submitting.value) {
    return
  }

  submitting.value = true
  submitState.value = 'pending'
  submitError.value = null

  try {
    await createPostComment(props.slug, author.value.trim(), content.value.trim())
    submitState.value = 'done'
    content.value = ''
  } catch (error) {
    submitState.value = 'error'
    submitError.value = error instanceof Error ? error.message : '发表失败'
  } finally {
    submitting.value = false
  }
}

watch(
  () => props.slug,
  () => {
    loadComments()
    submitState.value = 'idle'
    submitError.value = null
  },
  { immediate: true },
)

onMounted(loadComments)
</script>

<template>
  <section class="comments-section" aria-label="评论">
    <div class="comments-divider">
      <span></span>
      <span>COMMENTS · {{ count }}</span>
      <span></span>
    </div>

    <p v-if="loading" class="comments-state">加载评论中…</p>
    <p v-else-if="loadError" class="comments-state">评论加载失败，请稍后再试。</p>
    <p v-else-if="!count" class="comments-state">还没有评论，来说点什么吧。</p>

    <ul v-else class="comment-list">
      <li v-for="comment in comments" :key="comment.id" class="comment-item">
        <div class="comment-head">
          <span class="comment-prompt" aria-hidden="true">❯</span>
          <strong>{{ comment.author }}</strong>
          <time :datetime="comment.createdAt">{{ formatTime(comment.createdAt) }}</time>
        </div>
        <p class="comment-body">{{ comment.content }}</p>
      </li>
    </ul>

    <form class="comment-form" @submit.prevent="submit">
      <div class="comment-form-heading">写下你的评论</div>

      <div class="comment-form-grid">
        <label class="comment-field">
          <span>昵称</span>
          <input v-model="author" type="text" maxlength="40" placeholder="how you want to be called" />
        </label>
        <label class="comment-field comment-field--content">
          <span>内容</span>
          <textarea
            v-model="content"
            rows="3"
            maxlength="1000"
            placeholder="说点什么…（评论发表后需要审核通过才会显示）"
          ></textarea>
        </label>
      </div>

      <div class="comment-form-actions">
        <button class="comment-submit" type="submit" :disabled="submitting || !author.trim() || !content.trim()">
          {{ submitting ? '发表中…' : '发表评论' }}
        </button>

        <span v-if="submitState === 'done'" class="comment-hint comment-hint--ok">
          已提交，审核通过后会显示在这里。
        </span>
        <span v-else-if="submitState === 'error'" class="comment-hint comment-hint--error">
          {{ submitError }}
        </span>
      </div>
    </form>
  </section>
</template>

<style scoped>
.comments-section {
  margin-top: clamp(36px, 6vw, 56px);
}

.comments-divider {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 1.75rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.12em;
}

.comments-divider span:not(:nth-child(2)) {
  height: 1px;
  flex: 1;
  background: var(--border);
}

.comments-divider span:nth-child(2) {
  color: var(--accent);
}

.comments-state {
  margin: 0;
  padding: 1rem 0;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.comment-list {
  display: grid;
  margin: 0;
  padding: 0;
  gap: 0.9rem;
  list-style: none;
}

.comment-item {
  padding: 0.9rem 1.1rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.comment-head {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.comment-prompt {
  color: var(--accent);
}

.comment-head time {
  margin-left: auto;
  color: var(--text-muted);
  font-size: var(--text-label);
}

.comment-body {
  margin: 0.5rem 0 0;
  color: var(--text-soft);
  font-size: var(--text-small);
  line-height: 1.75;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}

.comment-form {
  margin-top: 2rem;
  padding: 1.1rem 1.2rem;
  border: 1px dashed var(--border-strong);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.comment-form-heading {
  margin-bottom: 0.9rem;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.comment-form-grid {
  display: grid;
  grid-template-columns: minmax(0, 220px) minmax(0, 1fr);
  gap: 0.75rem;
}

.comment-field {
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: 0.4rem;
}

.comment-field > span {
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.comment-field input,
.comment-field textarea {
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

.comment-field input {
  height: 2.5rem;
  padding: 0 0.65rem;
}

.comment-field textarea {
  padding: 0.6rem 0.65rem;
  resize: vertical;
}

.comment-field input:focus,
.comment-field textarea:focus {
  border-color: var(--accent-line);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.comment-form-actions {
  display: flex;
  margin-top: 0.9rem;
  align-items: center;
  gap: 0.75rem;
}

.comment-submit {
  min-height: 2.4rem;
  padding: 0.45rem 0.9rem;
  border: 0;
  border-radius: var(--radius-sm);
  color: var(--accent-contrast);
  background: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.comment-submit:hover:not(:disabled) {
  background: var(--accent-hover);
}

.comment-submit:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.comment-hint {
  font-size: var(--text-small);
}

.comment-hint--ok {
  color: var(--accent);
}

.comment-hint--error {
  color: var(--danger);
}

@media (max-width: 640px) {
  .comment-form-grid {
    grid-template-columns: 1fr;
  }
}
</style>
