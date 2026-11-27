<script setup lang="ts">
import { ref } from 'vue'
import { login } from '@/services/auth'

const emit = defineEmits<{
  success: []
}>()

const username = ref('')
const password = ref('')
const submitting = ref(false)
const error = ref<string | null>(null)

async function submit() {
  if (!username.value.trim() || !password.value || submitting.value) {
    return
  }

  submitting.value = true
  error.value = null

  try {
    await login(username.value.trim(), password.value)
    emit('success')
  } catch (loginError) {
    error.value = loginError instanceof Error ? loginError.message : '登录失败'
    password.value = ''
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <section class="login-page page-with-glow container">
    <form class="login-card" @submit.prevent="submit">
      <div class="login-bar">
        <span class="login-dots" aria-hidden="true">
          <i></i>
          <i></i>
          <i></i>
        </span>
        <span>sudo4 admin</span>
        <span>auth</span>
      </div>

      <div class="login-body">
        <span class="login-kicker">AUTH REQUIRED</span>
        <h1>登录后台</h1>
        <p>使用环境变量中配置的管理员账号登录，会话由服务端 JWT 保护。</p>

        <label class="login-field">
          <span>账号</span>
          <input v-model="username" type="text" autocomplete="username" spellcheck="false" />
        </label>

        <label class="login-field">
          <span>密码</span>
          <input v-model="password" type="password" autocomplete="current-password" />
        </label>

        <p v-if="error" class="login-error" role="alert">{{ error }}</p>

        <button class="login-submit" type="submit" :disabled="submitting || !username.trim() || !password">
          {{ submitting ? '登录中…' : '登录' }}
        </button>
      </div>
    </form>
  </section>
</template>

<style scoped>
.login-page {
  display: grid;
  min-height: calc(100svh - var(--header-height) - 12rem);
  padding-top: clamp(3rem, 8vh, 6rem);
  place-items: start center;
}

.login-card {
  width: min(26rem, 100%);
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background:
    radial-gradient(circle at 100% 0, var(--surface-glow), transparent 48%),
    var(--surface);
  box-shadow: var(--shadow-lg);
}

.login-bar {
  display: flex;
  height: 38px;
  align-items: center;
  justify-content: space-between;
  padding: 0 14px;
  border-bottom: 1px solid var(--border);
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.login-dots {
  display: flex;
  gap: 5px;
}

.login-dots i {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--border-strong);
}

.login-dots i:first-child {
  background: var(--accent);
}

.login-body {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.75rem;
}

.login-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.login-body h1 {
  font-size: 1.6rem;
  letter-spacing: -0.04em;
}

.login-body > p {
  margin-top: -0.5rem;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.7;
}

.login-field {
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.login-field > span {
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.login-field input {
  height: 2.7rem;
  width: 100%;
  padding: 0 0.7rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  outline: 0;
  color: var(--text);
  background: var(--surface-soft);
  font-size: var(--text-small);
}

.login-field input:focus {
  border-color: var(--accent-line);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.login-error {
  margin: 0;
  padding: 0.6rem 0.8rem;
  border: 1px solid var(--danger-line);
  border-radius: var(--radius-sm);
  color: var(--danger);
  background: var(--danger-soft);
  font-size: var(--text-small);
}

.login-submit {
  min-height: 2.7rem;
  border: 0;
  border-radius: var(--radius-sm);
  color: var(--accent-contrast);
  background: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-small);
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.login-submit:hover:not(:disabled) {
  background: var(--accent-hover);
}

.login-submit:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}
</style>
