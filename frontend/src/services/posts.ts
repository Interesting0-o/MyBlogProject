import type { Post } from '@/types/blog'
import { renderMarkdown } from '@/utils/markdown'

const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL || '/api').replace(/\/+$/, '')

let cachedPosts: Post[] | undefined
let pendingPosts: Promise<Post[]> | undefined

function clonePost(post: Post): Post {
  return {
    ...post,
    tags: [...post.tags],
  }
}

function renderRemotePostContent(post: Post): Post {
  return {
    ...clonePost(post),
    content: renderMarkdown(post.content),
  }
}

function normalizePosts(posts: Post[], renderContent = false): Post[] {
  return posts
    .map((post) => (renderContent ? renderRemotePostContent(post) : clonePost(post)))
    .sort((a, b) => b.publishedAt.localeCompare(a.publishedAt))
}

async function fetchRemotePosts(): Promise<Post[]> {
  const response = await fetch(`${apiBaseUrl}/posts`, {
    headers: {
      Accept: 'application/json',
    },
  })

  if (!response.ok) {
    throw new Error(`Failed to load posts: ${response.status}`)
  }

  const payload = (await response.json()) as Post[] | { posts?: Post[] }
  const posts = Array.isArray(payload) ? payload : payload.posts

  if (!posts) {
    throw new Error('Invalid posts response')
  }

  return normalizePosts(posts, true)
}

export async function fetchPosts(): Promise<Post[]> {
  if (cachedPosts) return cachedPosts

  pendingPosts ??= fetchRemotePosts()
    .then((posts) => {
      cachedPosts = posts
      return posts
    })
    .finally(() => {
      pendingPosts = undefined
    })

  return pendingPosts
}
