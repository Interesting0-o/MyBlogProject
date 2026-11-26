import { readonly, ref } from 'vue'
import { defaultSettings, fetchSiteSettings } from '@/services/settings'
import type { SiteSettings } from '@/types/blog'

const settings = ref<SiteSettings>({ ...defaultSettings })
let initialized = false

export function useSiteSettings() {
  if (!initialized) {
    initialized = true
    fetchSiteSettings().then((value) => {
      settings.value = value
    })
  }

  return { settings: readonly(settings) }
}
