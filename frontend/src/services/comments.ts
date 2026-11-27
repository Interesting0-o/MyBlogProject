import type { Comment } from '@/types/blog'

const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL || '/api').replace(/\/+$/, '')

export async function fetchPostComments(slug: string): Promise<Comment[]> {
  const response = await fetch(
    `${apiBaseUrl}/posts/${encodeURIComponent(slug)}/comments`,
    {
      headers: { Accept: 'application/json' },
    },
  )

  if (!response.ok) {
    throw new Error(`Failed to load comments: ${response.status}`)
  }

  const payload = (await response.json()) as { comments?: Comment[] }

  if (!payload.comments) {
    throw new Error('Invalid comments response')
  }

  return payload.comments
}

export async function createPostComment(
  slug: string,
  author: string,
  content: string,
): Promise<Comment> {
  const response = await fetch(
    `${apiBaseUrl}/posts/${encodeURIComponent(slug)}/comments`,
    {
      method: 'POST',
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ author, content }),
    },
  )

  if (!response.ok) {
    let message = `发表失败：${response.status}`

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

  return (await response.json()) as Comment
}
