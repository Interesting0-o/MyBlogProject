<script setup lang="ts">
const year = new Date().getFullYear()
</script>

<template>
  <footer class="site-footer">
    <div class="container footer-inner">
      <div class="footer-brand">
        <span class="footer-prompt" aria-hidden="true">❯</span>
        <span>© {{ year }} sudo4</span>
        <span class="footer-divider" aria-hidden="true">/</span>
        <span>built with curiosity</span>
      </div>

      <div class="footer-meta">
        <span class="footer-status">
          <span class="status-dot" aria-hidden="true"></span>
          all systems nominal
        </span>
        <span class="footer-divider" aria-hidden="true">/</span>
        <span>vue + vite</span>
        <span class="footer-divider footer-divider--desktop" aria-hidden="true">/</span>
        <span class="footer-hint footer-hint--desktop">press <kbd>T</kbd> to theme</span>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.site-footer {
  margin-top: clamp(4.5rem, 8vw, 7rem);
  border-top: 1px solid var(--border);
}

.footer-inner {
  display: flex;
  min-height: 5.5rem;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  padding-block: 1.4rem;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.02em;
}

.footer-brand,
.footer-meta,
.footer-status {
  display: flex;
  align-items: center;
}

.footer-brand {
  gap: 9px;
}

.footer-prompt {
  color: var(--accent);
}

.footer-meta {
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 10px;
}

.footer-status {
  gap: 7px;
}

.status-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 0 4px var(--accent-soft);
  animation: status-pulse 2.8s ease-in-out infinite;
}

.footer-divider {
  color: var(--border-strong);
}

.footer-hint {
  display: inline-flex;
  align-items: center;
  gap: 5px;
}

kbd {
  display: inline-grid;
  min-width: 18px;
  height: 18px;
  place-items: center;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--surface);
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1;
}

@keyframes status-pulse {
  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.45;
  }
}

@media (max-width: 760px) {
  .site-footer {
    margin-top: 64px;
  }

  .footer-inner {
    min-height: 100px;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    gap: 10px;
  }

  .footer-meta {
    justify-content: flex-start;
  }

  .footer-divider--desktop,
  .footer-hint--desktop {
    display: none;
  }
}
</style>
