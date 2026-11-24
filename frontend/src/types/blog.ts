export interface Post {
  slug: string
  title: string
  description: string
  publishedAt: string
  updatedAt?: string
  tags: string[]
  readingMinutes: number
  featured?: boolean
  content: string
}
