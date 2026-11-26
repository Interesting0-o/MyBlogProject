import { readonly, ref } from 'vue'

export type AdminTab =
  | 'posts'
  | 'compose'
  | 'drafts'
  | 'comments'
  | 'settings'
  | 'stats'

export type Route =
  | { name: 'home' }
  | { name: 'post'; slug: string }
  | { name: 'archive' }
  | { name: 'about' }
  | { name: 'admin'; tab: AdminTab }
  | { name: 'not-found' }

const route = ref<Route>({ name: 'home' })
let initialized = false

function parseAdminTab(tab: string): AdminTab {
  switch (tab) {
    case 'compose':
    case 'drafts':
    case 'comments':
    case 'settings':
    case 'stats':
      return tab
    default:
      return 'posts'
  }
}

export function parsePath(rawPath: string): Route {
  const path = rawPath
    .replace(/^\/+/, '')
    .replace(/\/+$/, '')

  if (!path) return { name: 'home' }
  if (path === 'archive') return { name: 'archive' }
  if (path === 'about') return { name: 'about' }

  if (path === 'admin' || path.startsWith('admin/')) {
    const tab = path === 'admin' ? '' : path.slice('admin/'.length)
    return { name: 'admin', tab: parseAdminTab(tab) }
  }

  const postMatch = /^posts\/([^/]+)$/.exec(path)
  if (postMatch?.[1]) {
    return { name: 'post', slug: postMatch[1] }
  }

  return { name: 'not-found' }
}

function syncRoute() {
  route.value = parsePath(window.location.pathname)
}

export function navigate(to: string) {
  const target = to.startsWith('/') ? to : `/${to}`

  if (window.location.pathname === target) {
    return
  }

  window.history.pushState({}, '', target)
  syncRoute()
}

// 兼容旧的 /#/xxx 链接：首次进入时把 hash 形式的路径迁移为普通路径。
function initialPath(): string {
  const { hash } = window.location

  if (hash.startsWith('#/')) {
    const path = hash.slice(1)
    window.history.replaceState({}, '', path)
    return path
  }

  return window.location.pathname
}

export function useRoute() {
  if (!initialized) {
    initialized = true
    route.value = parsePath(initialPath())

    window.addEventListener('popstate', syncRoute)
  }

  return { route: readonly(route) }
}
