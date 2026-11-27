<script setup lang="ts">
import { computed } from 'vue'
import AppLink from '@/components/AppLink.vue'
import { useRoute } from '@/composables/useRoute'
import { useSiteSettings } from '@/composables/useSiteSettings'
import { useTheme } from '@/composables/useTheme'

const { route } = useRoute()
const { isDark, toggleTheme } = useTheme()
const { settings } = useSiteSettings()

const navItems = [
  { index: '01', label: '首页', routeName: 'home', href: '/' },
  { index: '02', label: '归档', routeName: 'archive', href: '/archive' },
  { index: '03', label: '关于', routeName: 'about', href: '/about' },
] as const

const activeRoute = computed(() => route.value.name)
</script>

<template>
  <header class="site-header">
    <div class="container header-inner">
      <AppLink class="brand" href="/" :aria-label="`${settings.siteTitle} 首页`">
        <span class="brand-mark" aria-hidden="true">❯</span>
        <span>{{ settings.siteTitle }}</span>
        <span class="brand-cursor" aria-hidden="true">_</span>
      </AppLink>

      <nav class="site-nav" aria-label="主导航">
        <AppLink
          v-for="item in navItems"
          :key="item.routeName"
          :href="item.href"
          :class="{ active: activeRoute === item.routeName }"
          :aria-current="activeRoute === item.routeName ? 'page' : undefined"
        >
          <span class="nav-index">{{ item.index }}</span>
          <span>{{ item.label }}</span>
        </AppLink>
      </nav>

      <div class="header-actions">
        <span class="header-status">
          <span class="status-dot" aria-hidden="true"></span>
          online
        </span>
        <button
          class="theme-toggle"
          type="button"
          :aria-label="isDark ? '切换到浅色模式' : '切换到深色模式'"
          aria-keyshortcuts="T"
          :title="isDark ? '切换到浅色模式 (T)' : '切换到深色模式 (T)'"
          @click="toggleTheme"
        >
          <svg
            v-if="isDark"
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            stroke-width="1.7"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <circle cx="12" cy="12" r="4" />
            <path d="M12 2v2M12 20v2M4.93 4.93l1.42 1.42M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.42-1.42M17.66 6.34l1.41-1.41" />
          </svg>
          <svg
            v-else
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            stroke-width="1.7"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
          >
            <path d="M20.5 14.1A8.4 8.4 0 1 1 9.9 3.5a6.7 6.7 0 0 0 10.6 10.6Z" />
          </svg>
        </button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.site-header {
  position: sticky;
  top: 0;
  z-index: 40;
  border-bottom: 1px solid var(--border);
  background: var(--header-bg);
  backdrop-filter: blur(18px);
}

.header-inner {
  display: flex;
  height: var(--header-height);
  align-items: center;
  gap: 28px;
}

.brand {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  color: var(--text);
  font-family: var(--font-mono);
  font-size: 1.15rem;
  font-weight: 600;
  letter-spacing: -0.03em;
  white-space: nowrap;
}

.brand-mark {
  color: var(--accent);
  font-weight: 500;
}

.brand-cursor {
  margin-left: -5px;
  color: var(--accent);
  animation: cursor-blink 1.15s step-end infinite;
}

.site-nav {
  display: flex;
  margin-left: auto;
  align-items: center;
  gap: 4px;
}

.site-nav a {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  padding: 0.55rem 0.8rem;
  border-radius: var(--radius-sm);
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
  font-weight: 500;
  letter-spacing: 0.02em;
  transition:
    color 0.2s ease,
    background-color 0.2s ease;
}

.site-nav a:hover {
  color: var(--text);
  background: linear-gradient(135deg, var(--accent-soft), transparent);
}

.site-nav a.active {
  color: var(--accent);
  background: linear-gradient(135deg, var(--accent-soft), transparent 80%);
}

.nav-index {
  color: var(--text-muted);
  font-size: var(--text-label);
  opacity: 0.55;
}

.site-nav a.active .nav-index {
  color: var(--accent);
  opacity: 0.65;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 14px;
}

.header-status {
  display: none;
  align-items: center;
  gap: 7px;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.05em;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 0 4px var(--accent-soft);
}

.theme-toggle {
  display: grid;
  width: 2.5rem;
  height: 2.5rem;
  place-items: center;
  border: 1px solid var(--border);
  border-radius: 10px;
  color: var(--text-soft);
  background: var(--surface);
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease,
    transform 0.2s ease;
}

.theme-toggle:hover {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
  transform: rotate(-6deg);
}

@keyframes cursor-blink {
  0%,
  48% {
    opacity: 1;
  }

  49%,
  100% {
    opacity: 0;
  }
}

@media (min-width: 900px) {
  .header-status {
    display: inline-flex;
  }
}

@media (max-width: 640px) {
  .header-inner {
    gap: 12px;
  }

  .site-nav {
    gap: 0;
  }

  .site-nav a {
    gap: 5px;
    padding: 0.45rem 0.55rem;
  }

  .nav-index {
    display: none;
  }

  .header-actions {
    gap: 8px;
  }

  .theme-toggle {
    width: 2.35rem;
    height: 2.35rem;
  }
}

@media (max-width: 380px) {
  .brand {
    font-size: 1rem;
  }

  .site-nav a {
    padding-inline: 0.45rem;
    font-size: var(--text-label);
  }
}
</style>
