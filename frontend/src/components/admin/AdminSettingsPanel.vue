<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { getAdminSettings, updateAdminSettings } from '@/services/admin'
import { defaultSettings, invalidateSettingsCache } from '@/services/settings'
import type { AboutStackGroup, SiteSettings } from '@/types/blog'

type SettingsSection = 'general' | 'home' | 'about'

const loading = ref(true)
const saving = ref(false)
const message = ref<{ type: 'ok' | 'error'; text: string } | null>(null)
const section = ref<SettingsSection>('general')

const sections: { key: SettingsSection; label: string }[] = [
  { key: 'general', label: '基本' },
  { key: 'home', label: '首页' },
  { key: 'about', label: '关于页' },
]

const form = reactive<SiteSettings>({ ...defaultSettings })

function applySettings(settings: SiteSettings) {
  form.siteTitle = settings.siteTitle
  form.siteTagline = settings.siteTagline
  form.profileName = settings.profileName
  form.profileHandle = settings.profileHandle
  form.profileTagline = settings.profileTagline
  form.profileBio = settings.profileBio
  form.profileLinks = settings.profileLinks.map((link) => ({ ...link }))
  form.aboutMarkdown = settings.aboutMarkdown
  form.terminalHome = { ...settings.terminalHome }
  form.terminalAbout = { ...settings.terminalAbout }
  form.homeProfile = { ...settings.homeProfile }
  form.homeNow = { items: settings.homeNow.items.map((item) => ({ ...item })) }
  form.aboutNow = {
    heading: settings.aboutNow.heading,
    description: settings.aboutNow.description,
    items: settings.aboutNow.items.map((item) => ({ ...item })),
  }
  form.aboutPrinciples = {
    heading: settings.aboutPrinciples.heading,
    items: settings.aboutPrinciples.items.map((item) => ({ ...item })),
  }
  form.aboutStack = {
    heading: settings.aboutStack.heading,
    groups: settings.aboutStack.groups.map((group) => ({
      label: group.label,
      tags: [...group.tags],
    })),
  }
  form.aboutSite = { ...settings.aboutSite }
}

async function load() {
  loading.value = true

  try {
    applySettings(await getAdminSettings())
  } catch (error) {
    message.value = {
      type: 'error',
      text: error instanceof Error ? error.message : '加载设置失败',
    }
  } finally {
    loading.value = false
  }
}

async function save() {
  saving.value = true
  message.value = null

  try {
    const saved = await updateAdminSettings({ ...form, profileLinks: [...form.profileLinks] })
    applySettings(saved)
    invalidateSettingsCache()
    message.value = { type: 'ok', text: '设置已保存' }
  } catch (error) {
    message.value = {
      type: 'error',
      text: error instanceof Error ? error.message : '保存失败',
    }
  } finally {
    saving.value = false
  }
}

function addLink() {
  form.profileLinks.push({ kind: 'github', label: '', handle: '', href: '' })
}

function removeLink(index: number) {
  form.profileLinks.splice(index, 1)
}

function addNowItem() {
  form.homeNow.items.push({ key: '', value: '' })
}

function addAboutNowItem() {
  form.aboutNow.items.push({ key: '', value: '' })
}

function addPrinciple() {
  form.aboutPrinciples.items.push({ title: '', description: '' })
}

function addStackGroup() {
  form.aboutStack.groups.push({ label: '', tags: [] })
}

function updateStackTags(group: AboutStackGroup, event: Event) {
  const value = (event.target as HTMLInputElement).value
  group.tags = value
    .split(',')
    .map((tag) => tag.trim())
    .filter(Boolean)
}

onMounted(load)
</script>

<template>
  <section class="admin-panel settings-panel">
    <div class="panel-heading">
      <span>SITE SETTINGS</span>
    </div>

    <div v-if="!loading" class="section-nav">
      <button
        v-for="item in sections"
        :key="item.key"
        class="section-tab"
        :class="{ active: section === item.key }"
        type="button"
        @click="section = item.key"
      >
        {{ item.label }}
      </button>
    </div>

    <p v-if="loading" class="panel-state">加载中…</p>

    <template v-else>
      <template v-if="section === 'general'">
        <div class="field-grid">
          <label class="field">
            <span>站点名称</span>
            <input v-model="form.siteTitle" type="text" />
          </label>

          <label class="field">
            <span>站点标语</span>
            <input v-model="form.siteTagline" type="text" />
          </label>

          <label class="field">
            <span>显示名称</span>
            <input v-model="form.profileName" type="text" />
          </label>

          <label class="field">
            <span>社交账号 / Handle</span>
            <input v-model="form.profileHandle" type="text" />
          </label>

          <label class="field field--full">
            <span>个人标语</span>
            <input v-model="form.profileTagline" type="text" />
          </label>

          <label class="field field--full">
            <span>个人简介</span>
            <textarea v-model="form.profileBio" rows="3"></textarea>
          </label>
        </div>

        <div class="links-section">
          <div class="links-heading">
            <span>个人链接</span>
            <button class="action action--secondary action--small" type="button" @click="addLink">
              添加链接
            </button>
          </div>

          <p v-if="!form.profileLinks.length" class="links-empty">还没有配置链接。</p>

          <div v-for="(link, index) in form.profileLinks" :key="index" class="link-row">
            <label class="field">
              <span>类型</span>
              <select v-model="link.kind">
                <option value="github">GitHub</option>
                <option value="bilibili">Bilibili</option>
                <option value="repository">Repository</option>
                <option value="email">Email</option>
              </select>
            </label>
            <label class="field">
              <span>名称</span>
              <input v-model="link.label" type="text" />
            </label>
            <label class="field">
              <span>Handle</span>
              <input v-model="link.handle" type="text" />
            </label>
            <label class="field">
              <span>链接</span>
              <input v-model="link.href" type="text" spellcheck="false" />
            </label>
            <button class="action action--danger action--small link-remove" type="button" @click="removeLink(index)">
              移除
            </button>
          </div>
        </div>
      </template>

      <template v-else-if="section === 'home'">
        <section class="terminal-section">
          <div class="links-heading">
            <span>首页终端卡片</span>
          </div>

          <div class="terminal-grid">
            <label class="field">
              <span>窗口标题</span>
              <input v-model="form.terminalHome.title" type="text" />
            </label>
            <label class="field">
              <span>Shell 标签</span>
              <input v-model="form.terminalHome.shell" type="text" />
            </label>
            <label class="field">
              <span>whoami 名称</span>
              <input v-model="form.terminalHome.whoamiName" type="text" />
            </label>
            <label class="field">
              <span>whoami 身份</span>
              <input v-model="form.terminalHome.whoamiRole" type="text" />
            </label>
            <label class="field">
              <span>底部左侧</span>
              <input v-model="form.terminalHome.footerLeft" type="text" />
            </label>
            <label class="field">
              <span>底部右侧</span>
              <input v-model="form.terminalHome.footerRight" type="text" />
            </label>
          </div>
        </section>

        <section class="terminal-section">
          <div class="links-heading">
            <span>PROFILE 卡片</span>
          </div>

          <div class="terminal-grid">
            <label class="field">
              <span>用户名</span>
              <input v-model="form.homeProfile.username" type="text" />
            </label>
            <label class="field">
              <span>身份标签</span>
              <input v-model="form.homeProfile.role" type="text" />
            </label>
            <label class="field">
              <span>简介</span>
              <input v-model="form.homeProfile.bio" type="text" />
            </label>
          </div>
        </section>

        <section class="terminal-section">
          <div class="links-heading">
            <span>NOW 卡片</span>
            <button class="action action--secondary action--small" type="button" @click="addNowItem">
              添加条目
            </button>
          </div>

          <p v-if="!form.homeNow.items.length" class="links-empty">还没有 NOW 条目。</p>

          <div v-for="(item, index) in form.homeNow.items" :key="index" class="now-row">
            <label class="field">
              <span>标签</span>
              <input v-model="item.key" type="text" placeholder="building" />
            </label>
            <label class="field">
              <span>内容</span>
              <input v-model="item.value" type="text" placeholder="MyBlogProject" />
            </label>
            <button
              class="action action--danger action--small now-remove"
              type="button"
              @click="form.homeNow.items.splice(index, 1)"
            >
              移除
            </button>
          </div>
        </section>
      </template>

      <template v-else>
        <section class="terminal-section">
          <div class="links-heading">
            <span>identity 终端卡片</span>
          </div>

          <div class="terminal-grid">
            <label class="field">
              <span>窗口标题</span>
              <input v-model="form.terminalAbout.title" type="text" />
            </label>
            <label class="field">
              <span>whoami 名称</span>
              <input v-model="form.terminalAbout.whoamiName" type="text" />
            </label>
            <label class="field">
              <span>whoami 身份</span>
              <input v-model="form.terminalAbout.whoamiRole" type="text" />
            </label>
            <label class="field">
              <span>focus</span>
              <input v-model="form.terminalAbout.focus" type="text" />
            </label>
            <label class="field">
              <span>currently</span>
              <input v-model="form.terminalAbout.currently" type="text" />
            </label>
            <label class="field">
              <span>status</span>
              <input v-model="form.terminalAbout.status" type="text" />
            </label>
            <label class="field">
              <span>since</span>
              <input v-model="form.terminalAbout.since" type="text" />
            </label>
          </div>
        </section>

        <label class="field field--full">
          <span>关于页顶部 Markdown</span>
          <textarea
            v-model="form.aboutMarkdown"
            class="about-textarea"
            placeholder="支持 Markdown，显示在关于页顶部个人资料下方。"
            spellcheck="false"
          ></textarea>
        </label>

        <section class="terminal-section">
          <div class="links-heading">
            <span>01 / NOW 区块</span>
            <button class="action action--secondary action--small" type="button" @click="addAboutNowItem">
              添加条目
            </button>
          </div>

          <div class="terminal-grid">
            <label class="field">
              <span>标题</span>
              <input v-model="form.aboutNow.heading" type="text" />
            </label>
            <label class="field field--full">
              <span>说明</span>
              <input v-model="form.aboutNow.description" type="text" />
            </label>
          </div>

          <div v-for="(item, index) in form.aboutNow.items" :key="index" class="now-row">
            <label class="field">
              <span>标签</span>
              <input v-model="item.key" type="text" placeholder="BUILDING" />
            </label>
            <label class="field">
              <span>内容</span>
              <input v-model="item.value" type="text" />
            </label>
            <button
              class="action action--danger action--small now-remove"
              type="button"
              @click="form.aboutNow.items.splice(index, 1)"
            >
              移除
            </button>
          </div>
        </section>

        <section class="terminal-section">
          <div class="links-heading">
            <span>02 / PRINCIPLES 区块</span>
            <button class="action action--secondary action--small" type="button" @click="addPrinciple">
              添加条目
            </button>
          </div>

          <label class="field field--full">
            <span>标题</span>
            <input v-model="form.aboutPrinciples.heading" type="text" />
          </label>

          <div v-for="(item, index) in form.aboutPrinciples.items" :key="index" class="now-row">
            <label class="field">
              <span>原则</span>
              <input v-model="item.title" type="text" />
            </label>
            <label class="field">
              <span>说明</span>
              <input v-model="item.description" type="text" />
            </label>
            <button
              class="action action--danger action--small now-remove"
              type="button"
              @click="form.aboutPrinciples.items.splice(index, 1)"
            >
              移除
            </button>
          </div>
        </section>

        <section class="terminal-section">
          <div class="links-heading">
            <span>03 / STACK 区块</span>
            <button class="action action--secondary action--small" type="button" @click="addStackGroup">
              添加分组
            </button>
          </div>

          <label class="field field--full">
            <span>标题</span>
            <input v-model="form.aboutStack.heading" type="text" />
          </label>

          <p v-if="!form.aboutStack.groups.length" class="links-empty">还没有工具分组。</p>

          <div v-for="(group, index) in form.aboutStack.groups" :key="index" class="stack-edit-row">
            <label class="field">
              <span>分组名</span>
              <input v-model="group.label" type="text" placeholder="LANGUAGES" />
            </label>
            <label class="field">
              <span>标签（逗号分隔）</span>
              <input
                :value="group.tags.join(', ')"
                type="text"
                placeholder="Rust, TypeScript"
                @input="updateStackTags(group, $event)"
              />
            </label>
            <button
              class="action action--danger action--small now-remove"
              type="button"
              @click="form.aboutStack.groups.splice(index, 1)"
            >
              移除
            </button>
          </div>
        </section>

        <section class="terminal-section">
          <div class="links-heading">
            <span>04 / THIS SITE 区块</span>
          </div>

          <div class="terminal-grid">
            <label class="field">
              <span>标题</span>
              <input v-model="form.aboutSite.heading" type="text" />
            </label>
            <label class="field">
              <span>引言</span>
              <input v-model="form.aboutSite.quote" type="text" />
            </label>
            <label class="field field--full">
              <span>说明</span>
              <textarea v-model="form.aboutSite.description" rows="3"></textarea>
            </label>
          </div>
        </section>
      </template>

      <p v-if="message" class="form-message" :class="`form-message--${message.type}`">
        {{ message.text }}
      </p>

      <div class="form-actions">
        <button class="action action--publish" type="button" :disabled="saving" @click="save">
          {{ saving ? '保存中…' : '保存设置' }}
        </button>
        <span class="save-note">保存会提交所有分类的配置</span>
      </div>
    </template>
  </section>
</template>

<style scoped>
.admin-panel {
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.settings-panel {
  display: grid;
  gap: 1.5rem;
  padding: 1.35rem;
}

.panel-heading {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.04em;
}

.section-nav {
  display: flex;
  gap: 0.4rem;
  padding-bottom: 0.9rem;
  border-bottom: 1px solid var(--border);
}

.section-tab {
  padding: 0.4rem 0.85rem;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-soft);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  cursor: pointer;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.section-tab:hover {
  border-color: var(--accent-line);
  color: var(--accent);
}

.section-tab.active {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
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

.field textarea {
  padding: 0.8rem;
  resize: vertical;
}

.field input:focus,
.field select:focus,
.field textarea:focus {
  border-color: var(--accent-line);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.links-section,
.terminal-section {
  display: grid;
  gap: 0.9rem;
  padding: 1rem;
  border: 1px dashed var(--border-strong);
  border-radius: var(--radius-md);
}

.links-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.links-note {
  color: var(--text-muted);
  font-size: var(--text-label);
}

.links-empty {
  margin: 0;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.link-row {
  display: grid;
  grid-template-columns: 110px 1fr 1fr 1.4fr auto;
  align-items: end;
  gap: 0.6rem;
}

.link-remove,
.now-remove {
  margin-bottom: 2px;
}

.now-row {
  display: grid;
  grid-template-columns: 160px minmax(0, 1fr) auto;
  align-items: end;
  gap: 0.6rem;
}

.stack-edit-row {
  display: grid;
  grid-template-columns: 180px minmax(0, 1fr) auto;
  align-items: end;
  gap: 0.6rem;
}

.terminal-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 0.75rem;
}

.about-textarea {
  min-height: 10rem;
  font-family: var(--font-mono) !important;
  line-height: 1.75;
}

.form-message {
  margin: 0;
  padding: 0.6rem 0.8rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  font-size: var(--text-small);
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
  align-items: center;
  gap: 0.75rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border);
}

.save-note {
  color: var(--text-muted);
  font-size: var(--text-label);
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

.action--danger {
  color: var(--danger);
}

.action--danger:hover:not(:disabled) {
  border-color: var(--danger-line);
  color: var(--danger);
  background: var(--danger-soft);
}

.action--small {
  min-height: 2rem;
  padding: 0.3rem 0.6rem;
}

@media (max-width: 860px) {
  .field-grid {
    grid-template-columns: 1fr;
  }

  .link-row {
    grid-template-columns: 1fr 1fr;
  }

  .now-row,
  .stack-edit-row {
    grid-template-columns: 1fr;
  }

  .terminal-grid {
    grid-template-columns: 1fr;
  }
}
</style>
