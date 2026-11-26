<script setup lang="ts">
import { computed } from 'vue'
import ProfileLinkIcon from '@/components/ProfileLinkIcon.vue'
import ScrollCue from '@/components/ScrollCue.vue'
import type { ProfileLinkKind } from '@/content/profile'
import { useSiteSettings } from '@/composables/useSiteSettings'
import { renderMarkdown } from '@/utils/markdown'

const { settings } = useSiteSettings()

const aboutHtml = computed(() => renderMarkdown(settings.value.aboutMarkdown))

function isExternal(href: string): boolean {
  return href.startsWith('http')
}

function linkKind(kind: string): ProfileLinkKind {
  return (['github', 'bilibili', 'repository', 'email'] as const).includes(
    kind as ProfileLinkKind,
  )
    ? (kind as ProfileLinkKind)
    : 'repository'
}
</script>

<template>
  <section class="about-page page-with-glow container">
    <header class="about-hero">
      <div class="profile-panel">
        <div class="profile-header">
          <img
            class="profile-avatar"
            src="/avatar.jpg"
            :alt="`${settings.profileName} 的头像`"
            width="260"
            height="260"
          />

          <div class="profile-copy">
            <div class="page-kicker">ABOUT ME</div>
            <h1>
              <span class="profile-name">{{ settings.siteTitle }}</span>
              <span class="profile-handle">{{ settings.profileHandle }}</span>
            </h1>
            <p class="profile-tagline">{{ settings.profileTagline }}</p>
            <p class="profile-bio">{{ settings.profileBio }}</p>
          </div>
        </div>

        <div class="profile-links" aria-label="个人链接">
          <a
            v-for="link in settings.profileLinks"
            :key="link.label"
            class="profile-link"
            :class="{ 'profile-link--disabled': !link.href }"
            :href="link.href || undefined"
            :target="isExternal(link.href) ? '_blank' : undefined"
            :rel="isExternal(link.href) ? 'noopener noreferrer' : undefined"
            :aria-disabled="link.href ? undefined : 'true'"
            :title="link.handle"
          >
            <ProfileLinkIcon :kind="linkKind(link.kind)" />
            <span>{{ link.label }}</span>
          </a>
        </div>
      </div>

      <div class="about-terminal">
        <div class="identity-card">
          <div class="identity-bar">
            <span class="terminal-dots" aria-hidden="true">
              <i></i>
              <i></i>
              <i></i>
            </span>
            <span>{{ settings.terminalAbout.title }}</span>
          </div>
          <div class="identity-body">
            <p class="identity-command"><span aria-hidden="true">❯</span> whoami</p>
            <p class="identity-output">
              <span>{{ settings.terminalAbout.whoamiName }}</span>
              <span>— {{ settings.terminalAbout.whoamiRole }}</span>
            </p>
            <p class="identity-command"><span aria-hidden="true">❯</span> cat identity.yml</p>
            <dl>
              <div>
                <dt>focus</dt>
                <dd>{{ settings.terminalAbout.focus }}</dd>
              </div>
              <div>
                <dt>currently</dt>
                <dd>{{ settings.terminalAbout.currently }}</dd>
              </div>
              <div>
                <dt>status</dt>
                <dd class="identity-online"><i aria-hidden="true"></i> {{ settings.terminalAbout.status }}</dd>
              </div>
              <div>
                <dt>since</dt>
                <dd>{{ settings.terminalAbout.since }}</dd>
              </div>
            </dl>
            <p class="identity-command identity-caret">
              <span aria-hidden="true">❯</span><i aria-hidden="true"></i>
            </p>
          </div>
        </div>
      </div>

      <ScrollCue target="about-content" aria-label="滚动到关于内容" />
    </header>

      <div id="about-content" class="about-grid">
      <div class="about-main">
        <section v-if="aboutHtml" class="about-section">
          <span class="section-number">00 / INTRO</span>
          <h2>关于我</h2>
          <div class="about-markdown" v-html="aboutHtml"></div>
        </section>

        <section class="about-section">
          <span class="section-number">01 / NOW</span>
          <h2>{{ settings.aboutNow.heading }}</h2>
          <p>{{ settings.aboutNow.description }}</p>
          <ul class="now-list">
            <li v-for="item in settings.aboutNow.items" :key="item.key">
              <span class="now-key">{{ item.key }}</span>
              <span>{{ item.value }}</span>
            </li>
          </ul>
        </section>

        <section class="about-section">
          <span class="section-number">02 / PRINCIPLES</span>
          <h2>{{ settings.aboutPrinciples.heading }}</h2>
          <ol class="principle-list">
            <li v-for="(item, index) in settings.aboutPrinciples.items" :key="item.title">
              <span class="principle-index">{{ String(index + 1).padStart(2, '0') }}</span>
              <div>
                <h3>{{ item.title }}</h3>
                <p>{{ item.description }}</p>
              </div>
            </li>
          </ol>
        </section>

        <section class="about-section">
          <span class="section-number">03 / STACK</span>
          <h2>{{ settings.aboutStack.heading }}</h2>
          <div class="stack-groups">
            <div v-for="group in settings.aboutStack.groups" :key="group.label" class="stack-group">
              <span>{{ group.label }}</span>
              <div class="stack-tags">
                <span v-for="tag in group.tags" :key="tag">{{ tag }}</span>
              </div>
            </div>
          </div>
        </section>

        <section class="about-section">
          <span class="section-number">04 / THIS SITE</span>
          <h2>{{ settings.aboutSite.heading }}</h2>
          <p>{{ settings.aboutSite.description }}</p>
          <blockquote>{{ settings.aboutSite.quote }}</blockquote>
        </section>
      </div>

      <aside class="about-aside">
        <div class="quote-card">
          <span class="quote-label">MOTTO</span>
          <p>“Make it work,<br />make it right,<br />make it fast.”</p>
          <span class="quote-author">— Kent Beck</span>
        </div>
      </aside>
    </div>
  </section>
</template>

<style scoped>
.about-hero {
  --terminal-width: clamp(420px, 30vw, 460px);
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) var(--terminal-width);
  align-items: center;
  gap: clamp(2rem, 4vw, 4rem);
  min-height: calc(100vh - var(--header-height));
  min-height: calc(100svh - var(--header-height));
  padding-top: clamp(3rem, 7vh, 6rem);
  padding-bottom: clamp(5rem, 10vh, 8rem);
  border-bottom: 1px solid var(--border);
}

.profile-panel {
  display: flex;
  min-width: 0;
  flex-direction: column;
  align-items: flex-start;
}

.profile-header {
  display: flex;
  width: 100%;
  flex-wrap: wrap;
  align-items: center;
  gap: clamp(1.5rem, 3vw, 2.25rem);
}

.profile-avatar {
  width: clamp(200px, 22vw, 260px);
  height: auto;
  flex: 0 0 auto;
  aspect-ratio: 1;
  border: 1px solid var(--accent-line);
  border-radius: 50%;
  background: var(--accent-soft);
  box-shadow: 0 18px 50px var(--accent-glow);
  object-fit: cover;
}

.profile-copy {
  min-width: 0;
  max-width: 34rem;
  flex: 1 1 300px;
}

.page-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.about-hero h1 {
  display: flex;
  margin-top: 0.7rem;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.15rem;
}

.profile-name {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  font-weight: 500;
  letter-spacing: 0.04em;
}

.profile-handle {
  max-width: 100%;
  overflow-wrap: anywhere;
  color: var(--text);
  font-size: clamp(1.75rem, 2.9vw, 2.3rem);
  font-weight: 650;
  letter-spacing: -0.045em;
  line-height: 1.05;
}

.profile-tagline {
  margin-top: 0.75rem;
  color: var(--accent);
  font-size: clamp(0.95rem, 1.3vw, 1.05rem);
  font-weight: 500;
  line-height: 1.45;
}

.profile-bio {
  max-width: 42ch;
  margin-top: 0.6rem;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.8;
}

.profile-links {
  display: flex;
  width: 100%;
  margin-top: 1.5rem;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.profile-link {
  display: inline-flex;
  align-items: center;
  gap: 0.45rem;
  padding: 0.42rem 0.7rem;
  border: 1px solid var(--border);
  border-radius: 999px;
  color: var(--text-soft);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: 0.75rem;
  line-height: 1.4;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease,
    transform 0.2s ease;
}

.profile-link:hover {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
  transform: translateY(-1px);
}

.profile-link--disabled {
  color: var(--text-muted);
  cursor: not-allowed;
  opacity: 0.6;
}

.profile-link--disabled:hover {
  border-color: var(--border);
  color: var(--text-muted);
  background: var(--surface);
  transform: none;
}

.profile-link svg {
  flex: 0 0 auto;
  color: var(--text-muted);
  transition: color 0.2s ease;
}

.profile-link:hover svg {
  color: var(--accent);
}

.profile-link-pending {
  color: var(--text-muted);
  font-size: 0.68rem;
}

.about-terminal {
  min-width: 0;
  width: 100%;
}

.about-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 21rem;
  align-items: start;
  gap: clamp(2.5rem, 5vw, 5rem);
  padding-top: clamp(3rem, 6vw, 5rem);
  scroll-margin-top: var(--header-height);
}

.about-main {
  display: grid;
  gap: clamp(56px, 9vw, 88px);
}

.about-section h2 {
  margin-top: 10px;
  font-size: clamp(1.55rem, 3.2vw, 2rem);
}

.about-section > p {
  max-width: 62ch;
  margin-top: 17px;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.88;
}

.about-markdown {
  max-width: 62ch;
  margin-top: 17px;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.88;
}

.about-markdown :deep(h1),
.about-markdown :deep(h2) {
  margin: 1.4em 0 0.5em;
  color: var(--text);
  font-size: 1.15rem;
}

.about-markdown :deep(p) {
  margin: 0.8em 0;
}

.about-markdown :deep(a) {
  color: var(--accent);
  text-decoration: underline;
  text-underline-offset: 3px;
}

.about-markdown :deep(ul),
.about-markdown :deep(ol) {
  display: grid;
  margin: 0.8em 0;
  padding-left: 1.4em;
  gap: 0.3em;
}

.about-markdown :deep(code) {
  padding: 0.12em 0.36em;
  border-radius: 4px;
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: 0.86em;
}

.section-number {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.07em;
}

.now-list {
  display: grid;
  margin-top: 26px;
  gap: 0;
  list-style: none;
}

.now-list li {
  display: grid;
  grid-template-columns: 100px minmax(0, 1fr);
  gap: 18px;
  padding: 14px 0;
  border-bottom: 1px solid var(--border);
  color: var(--text-soft);
  font-size: var(--text-small);
}

.now-key {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.04em;
}

.principle-list {
  display: grid;
  margin-top: 26px;
  gap: 0;
  list-style: none;
}

.principle-list li {
  display: flex;
  gap: 18px;
  padding: 20px 0;
  border-bottom: 1px solid var(--border);
}

.principle-index {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1.9;
}

.principle-list h3 {
  font-size: 1rem;
  font-weight: 600;
  letter-spacing: -0.02em;
}

.principle-list p {
  margin-top: 7px;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.stack-groups {
  display: grid;
  margin-top: 24px;
  gap: 18px;
}

.stack-group {
  display: grid;
  grid-template-columns: 105px minmax(0, 1fr);
  align-items: start;
  gap: 18px;
}

.stack-group > span {
  padding-top: 4px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.05em;
}

.stack-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 7px;
}

.stack-tags span {
  padding: 3px 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-soft);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.about-section blockquote {
  max-width: 56ch;
  margin: 26px 0 0;
  padding: 16px 20px;
  border-left: 2px solid var(--accent);
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
  color: var(--text);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: var(--text-small);
}

.about-aside {
  position: sticky;
  top: calc(var(--header-height) + 28px);
  display: grid;
  gap: 14px;
}

.identity-card {
  display: flex;
  width: 100%;
  overflow: hidden;
  flex-direction: column;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background:
    radial-gradient(circle at 100% 0, var(--surface-glow), transparent 48%),
    var(--surface);
  box-shadow: var(--shadow-lg);
}

.identity-bar {
  display: flex;
  height: 36px;
  align-items: center;
  justify-content: space-between;
  padding: 0 13px;
  border-bottom: 1px solid var(--border);
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.terminal-dots {
  display: flex;
  gap: 5px;
}

.terminal-dots i {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--border-strong);
}

.terminal-dots i:first-child {
  background: var(--accent);
}

.identity-body {
  display: flex;
  flex: 1;
  flex-direction: column;
  padding: 24px;
}

.identity-command {
  margin: 0;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1.7;
}

.identity-command span {
  margin-right: 7px;
  color: var(--accent);
}

.identity-output {
  display: flex;
  margin-top: 4px;
  flex-wrap: wrap;
  align-items: baseline;
  column-gap: 0.35em;
  row-gap: 0.1em;
  color: var(--text);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
  font-weight: 600;
  letter-spacing: -0.01em;
  line-height: 1.6;
}

.identity-output span {
  white-space: nowrap;
}

.identity-output + .identity-command {
  margin-top: 22px;
}

.identity-body dl {
  display: grid;
  margin: 18px 0 0;
  gap: 10px;
}

.identity-body dl div {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
  padding-bottom: 9px;
  border-bottom: 1px solid var(--border);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.identity-body dl div:last-child {
  padding-bottom: 0;
  border-bottom: 0;
}

.identity-body dt {
  color: var(--text-muted);
}

.identity-body dd {
  margin: 0;
  color: var(--text-soft);
  text-align: right;
}

.identity-online {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  color: var(--accent) !important;
}

.identity-online i {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--accent);
}

.identity-caret {
  display: flex;
  align-items: center;
  margin-top: 20px;
}

.identity-caret i {
  display: inline-block;
  width: 7px;
  height: 13px;
  background: var(--accent);
  animation: caret-blink 1.1s step-end infinite;
}

.quote-card {
  padding: 20px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.quote-label {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.quote-card p {
  margin-top: 12px;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-small);
  line-height: 1.75;
}

.quote-author {
  display: block;
  margin-top: 12px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

@keyframes caret-blink {
  0%,
  48% {
    opacity: 1;
  }

  49%,
  100% {
    opacity: 0;
  }
}

@media (max-width: 1120px) {
  .about-hero {
    --terminal-width: clamp(340px, 36vw, 400px);
    gap: 2rem;
  }
}

@media (max-width: 720px) {
  .about-hero {
    grid-template-columns: 1fr;
    gap: 2.5rem;
  }

  .profile-header {
    flex-direction: column;
    align-items: center;
  }

  .profile-avatar {
    width: 180px;
  }

  .profile-copy {
    width: 100%;
    flex: 0 0 auto;
  }

  .about-terminal {
    width: min(calc(100% - 2rem), 28rem);
    margin-bottom: 2rem;
    justify-self: center;
  }

  .identity-bar {
    height: 34px;
    padding: 0 12px;
  }

  .identity-body {
    padding: 20px;
  }
}

@media (max-width: 900px) {
  .about-grid {
    grid-template-columns: 1fr;
  }

  .about-aside {
    position: static;
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .about-main {
    gap: 48px;
  }

  .now-list li {
    grid-template-columns: 1fr;
    gap: 5px;
  }

  .stack-group {
    grid-template-columns: 1fr;
    gap: 8px;
  }
}

</style>
