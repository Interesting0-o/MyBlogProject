import { marked } from 'marked'
import type { Post } from '@/types/blog'

const markdownModules = import.meta.glob<string>('../content/posts/*.md', {
  eager: true,
  import: 'default',
  query: '?raw',
})

function readFrontmatter(source: string, path: string) {
  const match = /^---\r?\n([\s\S]*?)\r?\n---\r?\n?([\s\S]*)$/.exec(source)

  if (!match) {
    throw new Error(`Missing frontmatter in ${path}`)
  }

  const frontmatter: Record<string, string> = {}

  for (const line of (match[1] ?? '').split(/\r?\n/)) {
    const separator = line.indexOf(':')
    if (separator <= 0) continue

    const key = line.slice(0, separator).trim()
    const value = line.slice(separator + 1).trim()

    if (key) frontmatter[key] = value
  }

  return {
    frontmatter,
    markdown: match[2] ?? '',
  }
}

function required(frontmatter: Record<string, string>, key: string, path: string): string {
  const value = frontmatter[key]

  if (!value) {
    throw new Error(`Missing "${key}" in ${path}`)
  }

  return value
}

function parseTags(value: string | undefined): string[] {
  if (!value) return []

  const list = value.startsWith('[') && value.endsWith(']') ? value.slice(1, -1) : value

  return list
    .split(',')
    .map((tag) => tag.trim().replace(/^['"]|['"]$/g, ''))
    .filter(Boolean)
}

function parsePost(path: string, source: string): Post {
  const { frontmatter, markdown } = readFrontmatter(source, path)
  const slug = path.split('/').at(-1)?.replace(/\.md$/, '')
  const tags = parseTags(frontmatter.tags)
  const readingMinutes = Number(required(frontmatter, 'readingMinutes', path))

  if (!slug) {
    throw new Error(`Cannot derive slug from ${path}`)
  }

  if (tags.length === 0) {
    throw new Error(`Missing "tags" in ${path}`)
  }

  if (!Number.isInteger(readingMinutes) || readingMinutes <= 0) {
    throw new Error(`Invalid "readingMinutes" in ${path}`)
  }

  return {
    slug,
    title: required(frontmatter, 'title', path),
    description: required(frontmatter, 'description', path),
    publishedAt: required(frontmatter, 'publishedAt', path),
    ...(frontmatter.updatedAt ? { updatedAt: frontmatter.updatedAt } : {}),
    tags,
    readingMinutes,
    featured: frontmatter.featured === 'true',
    content: marked.parse(markdown, { async: false, gfm: true }),
  }
}

export const mockPosts: Post[] = Object.entries(markdownModules)
  .map(([path, source]) => parsePost(path, source))
  .sort((a, b) => b.publishedAt.localeCompare(a.publishedAt))
