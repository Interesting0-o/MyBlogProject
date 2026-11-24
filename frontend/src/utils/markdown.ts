import DOMPurify from 'dompurify'
import { marked } from 'marked'

export function sanitizeHtml(html: string): string {
  return DOMPurify.sanitize(html)
}

export function renderMarkdown(markdown: string): string {
  return sanitizeHtml(marked.parse(markdown, { async: false, gfm: true }))
}
