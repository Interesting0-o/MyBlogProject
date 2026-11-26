import { readonly, ref } from 'vue'
import type { AdminPost } from '@/services/admin'

const editingPost = ref<AdminPost | null>(null)

export function useAdminPostStore() {
  return { editingPost: readonly(editingPost) }
}

export function setEditingPost(post: AdminPost) {
  editingPost.value = post
}

export function clearEditingPost() {
  editingPost.value = null
}
