import type { Comment, CommentStatus, Post, PostStatus, SiteSettings, StatsSummary } from '@/types/blog'

const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL || '/api').replace(/\/+$/, '')
const adminBaseUrl = `${apiBaseUrl}/admin`

export interface AdminPost extends Post {
  status: PostStatus
}

export interface AdminPostPayload {
  slug: string
  title: string
  description: string
  publishedAt: string
  updatedAt?: string
  tags: string[]
  readingMinutes: number
  featured: boolean
  status: PostStatus
  content: string
}

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const headers: Record<string, string> = {
    Accept: 'application/json',
    ...(options.body ? { 'Content-Type': 'application/json' } : {}),
  }

  const response = await fetch(`${adminBaseUrl}${path}`, {
    ...options,
    headers: { ...headers, ...options.headers },
  })

  if (!response.ok) {
    let message = `请求失败：${response.status}`

    try {
      const payload = (await response.json()) as { error?: string }

      if (payload.error) {
        message = payload.error
      }
    } catch {
      // 保留默认错误信息
    }

    throw new Error(message)
  }

  if (response.status === 204) {
    return undefined as T
  }

  return (await response.json()) as T
}

export async function listAdminPosts(): Promise<AdminPost[]> {
  const payload = await request<{ posts?: AdminPost[] }>('/posts')
  const posts = payload.posts

  if (!posts) {
    throw new Error('Invalid posts response')
  }

  return posts
}

export function getAdminPost(slug: string): Promise<AdminPost> {
  return request<AdminPost>(`/posts/${encodeURIComponent(slug)}`)
}

export function createAdminPost(payload: AdminPostPayload): Promise<AdminPost> {
  return request<AdminPost>('/posts', {
    method: 'POST',
    body: JSON.stringify(payload),
  })
}

export function updateAdminPost(slug: string, payload: AdminPostPayload): Promise<AdminPost> {
  return request<AdminPost>(`/posts/${encodeURIComponent(slug)}`, {
    method: 'PUT',
    body: JSON.stringify(payload),
  })
}

export async function deleteAdminPost(slug: string): Promise<void> {
  await request<void>(`/posts/${encodeURIComponent(slug)}`, {
    method: 'DELETE',
  })
}

export async function listAdminComments(status?: CommentStatus | 'all'): Promise<Comment[]> {
  const query = status && status !== 'all' ? `?status=${encodeURIComponent(status)}` : ''
  const payload = await request<{ comments?: Comment[] }>(`/comments${query}`)
  const comments = payload.comments

  if (!comments) {
    throw new Error('Invalid comments response')
  }

  return comments
}

export function setAdminCommentStatus(id: number, status: CommentStatus): Promise<Comment> {
  return request<Comment>(`/comments/${id}`, {
    method: 'PATCH',
    body: JSON.stringify({ status }),
  })
}

export async function deleteAdminComment(id: number): Promise<void> {
  await request<void>(`/comments/${id}`, {
    method: 'DELETE',
  })
}

export function getAdminSettings(): Promise<SiteSettings> {
  return request<SiteSettings>('/settings')
}

export function updateAdminSettings(settings: SiteSettings): Promise<SiteSettings> {
  return request<SiteSettings>('/settings', {
    method: 'PUT',
    body: JSON.stringify(settings),
  })
}

export function getAdminStats(): Promise<StatsSummary> {
  return request<StatsSummary>('/stats')
}
