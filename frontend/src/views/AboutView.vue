<script setup lang="ts">
import ScrollCue from '@/components/ScrollCue.vue'
</script>

<template>
  <section class="about-page page-with-glow container">
    <header class="about-hero">
      <div class="about-hero-copy">
        <div class="page-kicker">// ABOUT ME</div>
        <h1>
          保持好奇，<br />
          <span>持续构建。</span>
        </h1>
        <p class="about-lead">
          我是 sudo4，一名关注后端与基础设施的工程师。喜欢把复杂的系统拆开、理解，再用更简单的方式装回去。
          这里记录过程，也记录偶尔冒出来的想法。
        </p>
        <div class="about-actions">
          <a class="button button--primary" href="#/archive">浏览归档 <span aria-hidden="true">→</span></a>
          <a class="button button--ghost" href="#/">回到首页 <span aria-hidden="true">↩</span></a>
        </div>
      </div>

      <div class="about-hero-card">
        <div class="identity-card">
          <div class="identity-bar">
            <span class="terminal-dots" aria-hidden="true">
              <i></i>
              <i></i>
              <i></i>
            </span>
            <span>identity.yml</span>
          </div>
          <div class="identity-body">
            <span class="identity-avatar" aria-hidden="true">❯_</span>
            <h2>sudo4</h2>
            <p class="identity-role">backend engineer / writer</p>
            <dl>
              <div>
                <dt>focus</dt>
                <dd>backend &amp; infra</dd>
              </div>
              <div>
                <dt>currently</dt>
                <dd>building this blog</dd>
              </div>
              <div>
                <dt>status</dt>
                <dd class="identity-online"><i aria-hidden="true"></i> online</dd>
              </div>
              <div>
                <dt>since</dt>
                <dd>2026</dd>
              </div>
            </dl>
          </div>
        </div>
      </div>

      <ScrollCue target="about-content" aria-label="滚动到关于内容" />
    </header>

    <div id="about-content" class="about-grid">
      <div class="about-main">
        <section class="about-section">
          <span class="section-number">01 / NOW</span>
          <h2>现在</h2>
          <p>
            正在搭建这个博客的前后端，也在重新整理自己的知识库。比起追逐新的工具，更想把已经在用的东西理解得再深一点。
          </p>
          <ul class="now-list">
            <li>
              <span class="now-key">BUILDING</span>
              <span>MyBlogProject — Rust · Axum · PostgreSQL</span>
            </li>
            <li>
              <span class="now-key">LEARNING</span>
              <span>数据库内部机制与分布式系统设计</span>
            </li>
            <li>
              <span class="now-key">WRITING</span>
              <span>工程笔记、系统实践与一些长期问题</span>
            </li>
          </ul>
        </section>

        <section class="about-section">
          <span class="section-number">02 / PRINCIPLES</span>
          <h2>我相信的事</h2>
          <ol class="principle-list">
            <li>
              <span class="principle-index">01</span>
              <div>
                <h3>简单优先</h3>
                <p>能删掉的复杂度就不要留下。清晰的结构本身就是一种性能。</p>
              </div>
            </li>
            <li>
              <span class="principle-index">02</span>
              <div>
                <h3>记录过程</h3>
                <p>结果会被忘记，过程中的判断和取舍才是可以复用的资产。</p>
              </div>
            </li>
            <li>
              <span class="principle-index">03</span>
              <div>
                <h3>保持好奇</h3>
                <p>遇到不理解的东西，先停下来弄清楚它为什么存在。</p>
              </div>
            </li>
          </ol>
        </section>

        <section class="about-section">
          <span class="section-number">03 / STACK</span>
          <h2>常用工具</h2>
          <div class="stack-groups">
            <div class="stack-group">
              <span>LANGUAGES</span>
              <div class="stack-tags">
                <span>Rust</span>
                <span>TypeScript</span>
                <span>SQL</span>
                <span>Bash</span>
              </div>
            </div>
            <div class="stack-group">
              <span>SYSTEMS</span>
              <div class="stack-tags">
                <span>Linux</span>
                <span>PostgreSQL</span>
                <span>Redis</span>
                <span>Docker</span>
              </div>
            </div>
            <div class="stack-group">
              <span>DAILY</span>
              <div class="stack-tags">
                <span>Neovim</span>
                <span>tmux</span>
                <span>Git</span>
                <span>fzf</span>
              </div>
            </div>
          </div>
        </section>

        <section class="about-section">
          <span class="section-number">04 / THIS SITE</span>
          <h2>关于这个站点</h2>
          <p>
            这是一个极简的个人空间，用 Vue 3、TypeScript 和 Vite 构建。当前阶段先以静态数据驱动页面，等后端接口就绪后，
            会把数据层替换成真实的 API。界面保持克制，把注意力留给文字。
          </p>
          <blockquote>先让它跑起来，再让它变好。</blockquote>
        </section>
      </div>

      <aside class="about-aside">
        <div class="quote-card">
          <span class="quote-label">// MOTTO</span>
          <p>“Make it work,<br />make it right,<br />make it fast.”</p>
          <span class="quote-author">— Kent Beck</span>
        </div>

        <div class="terminal-card">
          <p><span>❯</span> contact --list</p>
          <p class="terminal-muted">这个站点还在生长，联系入口会在后续开放。</p>
          <p class="terminal-caret-line"><span>❯</span><i aria-hidden="true"></i></p>
        </div>
      </aside>
    </div>
  </section>
</template>

<style scoped>
.about-hero {
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) clamp(340px, 28vw, 420px);
  align-items: center;
  gap: clamp(2.5rem, 5vw, 5rem);
  min-height: calc(100vh - var(--header-height));
  min-height: calc(100svh - var(--header-height));
  padding-top: clamp(2rem, 5vh, 4rem);
  padding-bottom: clamp(4.5rem, 9vh, 7rem);
  border-bottom: 1px solid var(--border);
}

.about-hero-card {
  min-width: 0;
}

.page-kicker {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.about-hero h1 {
  margin-top: 1rem;
  font-size: clamp(2.8rem, 5.7vw, 5rem);
  letter-spacing: -0.06em;
  line-height: 1.06;
}

.about-hero h1 span {
  color: var(--gradient-accent-a);
  background: linear-gradient(
    100deg,
    var(--gradient-accent-a) 0%,
    var(--gradient-accent-b) 52%,
    var(--gradient-accent-c) 100%
  );
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.about-lead {
  max-width: 50ch;
  margin-top: 1.5rem;
  color: var(--text-soft);
  font-size: clamp(var(--text-lead), 1.4vw, 1.2rem);
  line-height: 1.9;
}

.about-actions {
  display: flex;
  margin-top: 2rem;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.button {
  display: inline-flex;
  min-height: 3rem;
  align-items: center;
  justify-content: center;
  gap: 0.6rem;
  padding: 0.72rem 1.25rem;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: var(--text-meta);
  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    border-color 0.2s ease,
    box-shadow 0.2s ease,
    transform 0.2s ease;
}

.button:hover {
  transform: translateY(-1px);
}

.button--primary {
  color: var(--accent-contrast);
  background: linear-gradient(120deg, var(--accent), var(--accent-hover));
  box-shadow: 0 12px 32px var(--accent-glow);
}

.button--primary:hover {
  box-shadow: 0 16px 42px var(--accent-glow);
}

.button--ghost {
  border-color: var(--border);
  color: var(--text-soft);
  background: var(--surface);
}

.button--ghost:hover {
  border-color: var(--accent-line);
  color: var(--accent);
  background: var(--accent-soft);
}

.about-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 21rem;
  align-items: start;
  gap: clamp(2rem, 5vw, 4.5rem);
  padding-top: clamp(2rem, 4vw, 3.5rem);
  scroll-margin-top: var(--header-height);
}

.about-main {
  display: grid;
  gap: clamp(48px, 8vw, 72px);
}

.about-section h2 {
  margin-top: 10px;
  font-size: clamp(1.55rem, 3.2vw, 2rem);
}

.about-section > p {
  max-width: 62ch;
  margin-top: 17px;
  color: var(--text-muted);
  font-size: var(--text-small);
  line-height: 1.88;
}

.section-number {
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.07em;
}

.now-list {
  display: grid;
  margin-top: 26px;
  gap: 0;
  list-style: none;
}

.now-list li {
  display: grid;
  grid-template-columns: 100px minmax(0, 1fr);
  gap: 18px;
  padding: 14px 0;
  border-bottom: 1px solid var(--border);
  color: var(--text-soft);
  font-size: var(--text-small);
}

.now-key {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.04em;
}

.principle-list {
  display: grid;
  margin-top: 26px;
  gap: 0;
  list-style: none;
}

.principle-list li {
  display: flex;
  gap: 18px;
  padding: 20px 0;
  border-bottom: 1px solid var(--border);
}

.principle-index {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1.9;
}

.principle-list h3 {
  font-size: 1rem;
  font-weight: 600;
  letter-spacing: -0.02em;
}

.principle-list p {
  margin-top: 7px;
  color: var(--text-muted);
  font-size: var(--text-small);
}

.stack-groups {
  display: grid;
  margin-top: 24px;
  gap: 18px;
}

.stack-group {
  display: grid;
  grid-template-columns: 105px minmax(0, 1fr);
  align-items: start;
  gap: 18px;
}

.stack-group > span {
  padding-top: 4px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.05em;
}

.stack-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 7px;
}

.stack-tags span {
  padding: 3px 8px;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-soft);
  background: var(--surface);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.about-section blockquote {
  max-width: 56ch;
  margin: 26px 0 0;
  padding: 16px 20px;
  border-left: 2px solid var(--accent);
  border-radius: 0 var(--radius-md) var(--radius-md) 0;
  color: var(--text);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: var(--text-small);
}

.about-aside {
  position: sticky;
  top: calc(var(--header-height) + 28px);
  display: grid;
  gap: 14px;
}

.identity-card {
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background:
    radial-gradient(circle at 100% 0, var(--surface-glow), transparent 48%),
    var(--surface);
  box-shadow: var(--shadow-lg);
}

.identity-bar {
  display: flex;
  height: 36px;
  align-items: center;
  justify-content: space-between;
  padding: 0 13px;
  border-bottom: 1px solid var(--border);
  color: var(--text-muted);
  background: var(--surface-soft);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.terminal-dots {
  display: flex;
  gap: 5px;
}

.terminal-dots i {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--border-strong);
}

.terminal-dots i:first-child {
  background: var(--accent);
}

.identity-body {
  padding: 22px;
}

.identity-avatar {
  display: grid;
  width: 44px;
  height: 44px;
  margin-bottom: 16px;
  place-items: center;
  border: 1px solid var(--accent-line);
  border-radius: 10px;
  color: var(--accent);
  background: var(--accent-soft);
  font-family: var(--font-mono);
  font-size: var(--text-small);
}

.identity-body h2 {
  font-family: var(--font-mono);
  font-size: 1.25rem;
  font-weight: 600;
  letter-spacing: -0.03em;
}

.identity-role {
  margin-top: 3px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.identity-body dl {
  display: grid;
  margin: 20px 0 0;
  gap: 10px;
}

.identity-body dl div {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
  padding-bottom: 9px;
  border-bottom: 1px solid var(--border);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.identity-body dl div:last-child {
  padding-bottom: 0;
  border-bottom: 0;
}

.identity-body dt {
  color: var(--text-muted);
}

.identity-body dd {
  margin: 0;
  color: var(--text-soft);
  text-align: right;
}

.identity-online {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  color: var(--accent) !important;
}

.identity-online i {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--accent);
}

.quote-card {
  padding: 20px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface);
}

.quote-label {
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  letter-spacing: 0.08em;
}

.quote-card p {
  margin-top: 12px;
  color: var(--text-soft);
  font-family: var(--font-mono);
  font-size: var(--text-small);
  line-height: 1.75;
}

.quote-author {
  display: block;
  margin-top: 12px;
  color: var(--text-muted);
  font-family: var(--font-mono);
  font-size: var(--text-label);
}

.terminal-card {
  padding: 18px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--surface-raised);
  font-family: var(--font-mono);
  font-size: var(--text-label);
  line-height: 1.75;
}

.terminal-card span {
  margin-right: 7px;
  color: var(--accent);
}

.terminal-muted {
  margin-top: 7px;
  color: var(--text-muted);
}

.terminal-caret-line {
  margin-top: 10px;
}

.terminal-caret-line i {
  display: inline-block;
  width: 7px;
  height: 13px;
  margin-bottom: -2px;
  background: var(--accent);
  animation: caret-blink 1.1s step-end infinite;
}

@keyframes caret-blink {
  0%,
  48% {
    opacity: 1;
  }

  49%,
  100% {
    opacity: 0;
  }
}

@media (max-width: 960px) {
  .about-hero {
    grid-template-columns: 1fr;
    gap: 2.5rem;
  }

  .about-hero-card {
    width: 100%;
    max-width: 28rem;
  }
}

@media (max-width: 900px) {
  .about-grid {
    grid-template-columns: 1fr;
  }

  .about-aside {
    position: static;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 720px) {
  .about-aside {
    grid-template-columns: 1fr;
  }

  .about-main {
    gap: 48px;
  }

  .now-list li {
    grid-template-columns: 1fr;
    gap: 5px;
  }

  .stack-group {
    grid-template-columns: 1fr;
    gap: 8px;
  }
}

@media (max-width: 420px) {
  .about-actions .button {
    flex: 1;
  }
}
</style>
