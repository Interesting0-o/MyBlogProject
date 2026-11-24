import { mockPosts } from '@/data/posts'
import type { Post } from '@/types/blog'

const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL || '/api').replace(/\/+$/, '')
const useMockData = import.meta.env.VITE_USE_MOCK_DATA !== 'false'

function clonePost(post: Post): Post {
  return {
    ...post,
    tags: [...post.tags],
  }
}

function normalizePosts(posts: Post[]): Post[] {
  return posts
    .map(clonePost)
    .sort((a, b) => b.publishedAt.localeCompare(a.publishedAt))
}

async function fetchMockPosts(): Promise<Post[]> {
  return normalizePosts(mockPosts)
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

  return normalizePosts(posts)
}

export async function fetchPosts(): Promise<Post[]> {
  return useMockData ? fetchMockPosts() : fetchRemotePosts()
}
