<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    target: string
    label?: string
  }>(),
  {
    label: 'SCROLL',
  },
)

function scrollToTarget() {
  const target = document.getElementById(props.target)

  if (!target) return

  const behavior = window.matchMedia('(prefers-reduced-motion: reduce)').matches
    ? 'auto'
    : 'smooth'

  target.scrollIntoView({ behavior })
}
</script>

<template>
  <button class="scroll-cue" type="button" @click="scrollToTarget">
    <span>{{ label }}</span>
    <span class="scroll-cue-line" aria-hidden="true"></span>
  </button>
</template>

<style scoped>
.scroll-cue {
  position: absolute;
  z-index: 2;
  bottom: clamp(1.25rem, 3vh, 2rem);
  left: 50%;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.55rem;
  padding: 0;
  color: var(--text-muted);
  background: transparent;
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.18em;
  transform: translateX(-50%);
  transition: color 0.2s ease;
}

.scroll-cue:hover {
  color: var(--accent);
}

.scroll-cue-line {
  position: relative;
  width: 1px;
  height: 2.5rem;
  overflow: hidden;
  background: linear-gradient(var(--border-strong), transparent);
}

.scroll-cue-line::after {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 45%;
  background: linear-gradient(transparent, var(--accent));
  content: '';
  animation: scroll-cue-move 1.8s ease-in-out infinite;
}

@keyframes scroll-cue-move {
  0% {
    opacity: 0;
    transform: translateY(-100%);
  }

  35% {
    opacity: 1;
  }

  100% {
    opacity: 0;
    transform: translateY(230%);
  }
}
</style>
