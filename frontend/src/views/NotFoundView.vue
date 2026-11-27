<template>
  <section class="not-found-page container">
    <div class="not-found-card">
      <div class="not-found-bar">
        <span class="terminal-dots" aria-hidden="true">
          <i></i>
          <i></i>
          <i></i>
        </span>
        <span>404 — route not found</span>
      </div>
      <div class="not-found-body">
        <span class="error-code">404</span>
        <p class="error-command">
          <span aria-hidden="true">❯</span>
          cd /this-page
        </p>
        <h1>这里什么都没有。</h1>
        <p class="error-copy">你访问的路径可能已经被移动，或者从未存在过。</p>
        <a class="not-found-link" href="#/">cd ~ →</a>
      </div>
    </div>
  </section>
</template>

<style scoped>
.not-found-page {
  display: flex;
  min-height: 68vh;
  align-items: center;
  justify-content: center;
  padding-top: 48px;
}

.not-found-card {
  width: min(620px, 100%);
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  background: var(--surface);
  box-shadow: var(--shadow-sm);
}

.not-found-bar {
  display: flex;
  height: 40px;
  align-items: center;
  justify-content: space-between;
  padding: 0 15px;
  border-bottom: 1px solid var(--border);
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.terminal-dots {
  display: flex;
  gap: 6px;
}

.terminal-dots i {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-strong);
}

.terminal-dots i:first-child {
  background: var(--accent);
}

.not-found-body {
  padding: clamp(32px, 6vw, 56px);
}

.error-code {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.16em;
}

.error-command {
  margin-top: 24px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
}

.error-command span {
  margin-right: 8px;
  color: var(--accent);
}

.not-found-body h1 {
  margin-top: 14px;
  font-size: clamp(2rem, 5vw, 3.1rem);
  letter-spacing: -0.055em;
}

.error-copy {
  max-width: 42ch;
  margin-top: 14px;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.not-found-link {
  display: inline-flex;
  margin-top: 28px;
  padding: 9px 14px;
  border: 1px solid var(--accent-line);
  border-radius: var(--radius-sm);
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
}

.not-found-link:hover {
  color: var(--accent-contrast);
  background: var(--accent);
}
</style>
