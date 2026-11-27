import { readonly, ref } from 'vue'

export type Route =
  | { name: 'home' }
  | { name: 'post'; slug: string }
  | { name: 'archive' }
  | { name: 'about' }
  | { name: 'admin' }
  | { name: 'not-found' }

const route = ref<Route>({ name: 'home' })
let initialized = false

function parseRoute(hash: string): Route {
  const path = hash
    .replace(/^#/, '')
    .replace(/^\/+/, '')
    .replace(/\/+$/, '')

  if (!path) return { name: 'home' }
  if (path === 'archive') return { name: 'archive' }
  if (path === 'about') return { name: 'about' }
  if (path === 'admin') return { name: 'admin' }

  const postMatch = /^posts\/([^/]+)$/.exec(path)
  if (postMatch?.[1]) {
    return { name: 'post', slug: postMatch[1] }
  }

  return { name: 'not-found' }
}

export function useRoute() {
  if (!initialized) {
    initialized = true
    route.value = parseRoute(window.location.hash)

    window.addEventListener('hashchange', () => {
      route.value = parseRoute(window.location.hash)
    })
  }

  return { route: readonly(route) }
}
