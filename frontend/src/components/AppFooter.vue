<script setup lang="ts">
import { profile } from '@/content/profile'
import { useTheme } from '@/composables/useTheme'

const year = new Date().getFullYear()
const { isDark, toggleTheme } = useTheme()

const navItems = [
  { label: '首页', href: '#/' },
  { label: '归档', href: '#/archive' },
  { label: '关于', href: '#/about' },
  { label: '发布', href: '#/admin' },
]

const contactLinks = profile.links.filter((link) => link.href)

function isExternal(href: string): boolean {
  return href.startsWith('http')
}

function scrollToTop() {
  const behavior = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    ? 'auto'
    : 'smooth'

  window.scrollTo({ top: 0, behavior })
}
</script>

<template>
  <footer class="site-footer">
    <div class="container footer-main">
      <div class="footer-brand-block">
        <a class="footer-logo" href="#/" aria-label="sudo4 首页">
          <span aria-hidden="true">❯</span>
          {{ profile.name }}<span class="footer-cursor" aria-hidden="true">_</span>
        </a>
        <p class="footer-tagline">{{ profile.tagline }}</p>
      </div>

      <nav class="footer-column" aria-label="站内导航">
        <span class="footer-heading">// SITEMAP</span>
        <a v-for="item in navItems" :key="item.href" :href="item.href">{{ item.label }}</a>
      </nav>

      <div class="footer-column">
        <span class="footer-heading">// ELSEWHERE</span>
        <a
          v-for="link in contactLinks"
          :key="link.label"
          :href="link.href"
          :target="isExternal(link.href) ? '_blank' : undefined"
          :rel="isExternal(link.href) ? 'noopener noreferrer' : undefined"
        >
          {{ link.label }}
          <span aria-hidden="true">{{ isExternal(link.href) ? '↗' : '→' }}</span>
        </a>
      </div>

      <div class="footer-column">
        <span class="footer-heading">// DISPLAY</span>
        <button
          class="footer-action"
          type="button"
          :aria-label="isDark ? '切换到浅色模式' : '切换到深色模式'"
          aria-keyshortcuts="T"
          @click="toggleTheme"
        >
          {{ isDark ? 'Light mode' : 'Dark mode' }}
          <span aria-hidden="true">◐</span>
        </button>
        <span class="footer-note">press <kbd>T</kbd> to theme</span>
        <button class="footer-action" type="button" @click="scrollToTop">
          Back to top
          <span aria-hidden="true">↑</span>
        </button>
      </div>
    </div>

    <div class="footer-bottom">
      <div class="container footer-bottom-inner">
        <span>© {{ year }} sudo4</span>
        <span class="footer-stack">Vue · Axum · SQLite</span>
        <span>built with curiosity</span>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.site-footer {
  margin-top: clamp(5rem, 9vw, 8rem);
  border-top: 1px solid var(--border);
  background:
    radial-gradient(circle at 10% 0, var(--accent-soft), transparent 34%),
    linear-gradient(180deg, transparent, var(--surface-soft));
}

.footer-main {
  display: grid;
  grid-template-columns: minmax(0, 1.4fr) repeat(3, minmax(0, 0.6fr));
  gap: clamp(2rem, 5vw, 4.5rem);
  padding-block: clamp(3rem, 6vw, 5rem);
}

.footer-brand-block {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.footer-logo {
  display: inline-flex;
  align-items: baseline;
  gap: 7px;
  color: var(--text);
  font-family: var(--font-mono);
  font-size: clamp(1.4rem, 2.4vw, 1.9rem);
  font-weight: 600;
  letter-spacing: -0.03em;
}

.footer-logo > span:first-child {
  color: var(--accent);
}

.footer-cursor {
  color: var(--accent);
  animation: footer-cursor-blink 1.15s step-end infinite;
}

.footer-tagline {
  max-width: 28ch;
  margin-top: 0.8rem;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.8;
}

.footer-column {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.55rem;
}

.footer-heading {
  margin-bottom: 0.45rem;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.footer-column a,
.footer-action {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 0;
  border: 0;
  color: var(--text-muted);
  background: none;
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1.7;
  cursor: pointer;
  transition:
    color 0.2s ease,
    transform 0.2s ease;
}

.footer-column a:hover,
.footer-action:hover {
  color: var(--accent);
  transform: translateX(2px);
}

.footer-action span {
  color: var(--accent);
}

.footer-note {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1.7;
}

kbd {
  display: inline-grid;
  min-width: 18px;
  height: 18px;
  place-items: center;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--surface);
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1;
}

.footer-bottom {
  border-top: 1px solid var(--border);
}

.footer-bottom-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding-block: 1.15rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.footer-stack {
  color: var(--text-soft);
}

@keyframes footer-cursor-blink {
  0%,
  48% {
    opacity: 1;
  }

  49%,
  100% {
    opacity: 0;
  }
}

@media (max-width: 960px) {
  .footer-main {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .footer-brand-block {
    grid-column: 1 / -1;
  }
}

@media (max-width: 720px) {
  .footer-main {
    grid-template-columns: 1fr;
    gap: 2rem;
  }

  .footer-brand-block {
    grid-column: auto;
  }

  .footer-bottom-inner {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.35rem;
  }
}
</style>
