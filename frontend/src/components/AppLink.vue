<script setup lang="ts">
import { navigate } from '@/composables/useRoute'

const props = defineProps<{
  href: string
}>()

function handleClick(event: MouseEvent) {
  if (
    event.defaultPrevented ||
    event.button !== 0 ||
    event.metaKey ||
    event.ctrlKey ||
    event.shiftKey ||
    event.altKey
  ) {
    return
  }

  event.preventDefault()
  navigate(props.href)
}
</script>

<template>
  <a :href="href" @click="handleClick">
    <slot></slot>
  </a>
</template>
