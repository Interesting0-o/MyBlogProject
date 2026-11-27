import { computed, readonly, ref } from 'vue'

export type Theme = 'light' | 'dark'

const STORAGE_KEY = 'sudo4-theme'
const theme = ref<Theme>('light')
let initialized = false

function applyTheme(next: Theme) {
  theme.value = next
  document.documentElement.dataset.theme = next

  const meta = document.querySelector<HTMLMetaElement>('meta[name="theme-color"]')
  if (meta) {
    meta.content = next === 'dark' ? '#080d0c' : '#f4f7f6'
  }
}

function detectTheme(): Theme {
  const current = document.documentElement.dataset.theme
  if (current === 'light' || current === 'dark') return current

  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

export function useTheme() {
  if (!initialized) {
    initialized = true
    applyTheme(detectTheme())

    window.addEventListener('storage', (event) => {
      if (
        event.key === STORAGE_KEY &&
        (event.newValue === 'light' || event.newValue === 'dark')
      ) {
        applyTheme(event.newValue)
      }
    })
  }

  const isDark = computed(() => theme.value === 'dark')

  function toggleTheme() {
    const next: Theme = theme.value === 'dark' ? 'light' : 'dark'
    applyTheme(next)

    try {
      localStorage.setItem(STORAGE_KEY, next)
    } catch {
      return
    }
  }

  return {
    theme: readonly(theme),
    isDark,
    toggleTheme,
  }
}
