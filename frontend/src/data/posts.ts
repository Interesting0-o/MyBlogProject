import type { Post } from '@/types/blog'

export const mockPosts: Post[] = [
  {
    slug: 'rewriting-blog-with-rust-axum',
    title: '用 Rust 和 Axum 重写博客后端',
    description: '从一个空的 Cargo 项目开始，把路由、配置和数据层一层层搭起来。',
    publishedAt: '2026-11-18',
    updatedAt: '2026-11-20',
    tags: ['Rust', 'Backend', 'Axum'],
    readingMinutes: 8,
    featured: true,
    content: `
      <p>这个博客的前端已经先跑了起来，而后端还停在一行 <code>hello, world</code>。这次我决定不再从熟悉的框架开始，而是用 Rust 把整条链路重新搭一遍。</p>
      <h2>为什么是 Axum？</h2>
      <p>Axum 足够轻，路由、提取器和响应类型都贴着标准库与 Tower 生态。它不急着替我决定项目结构，也不会用一层又一层的隐藏状态把问题包起来。</p>
      <pre><code>use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hello, from sudo4" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}</code></pre>
      <p>先让最小的服务跑起来，确认端口、日志和错误处理都在预期之内，再逐步加上数据库、鉴权与缓存。顺序比速度重要。</p>
      <h2>先把边界画清楚</h2>
      <p>我给项目定下了三条约束：</p>
      <ul>
        <li>配置只从环境变量进入，不散落在业务代码里。</li>
        <li>数据库访问收敛到一个模块，handler 不直接拼 SQL。</li>
        <li>所有对外响应都有明确的结构，错误也不例外。</li>
      </ul>
      <p>这些约束不会让第一版更漂亮，但会让第二版更容易继续。</p>
      <blockquote><p>能用一晚上写出来的东西，也值得用一周时间想清楚它应该如何生长。</p></blockquote>
      <h2>下一步</h2>
      <p>等文章与项目接口稳定后，我会把前端的数据层切到真实 API。到那时，现在这些死数据会变成数据库里的种子，而不是被丢掉。</p>
      <p>构建仍然继续。终端还开着。</p>
    `,
  },
  {
    slug: 'css-variable-theming',
    title: '用 CSS 变量做一套不闪眼的双主题',
    description: '颜色不只是两套色值。先定义语义，再让光照进来。',
    publishedAt: '2026-11-06',
    tags: ['CSS', 'Design', 'Frontend'],
    readingMinutes: 6,
    content: `
      <p>双主题最容易犯的错误，是先写一套浅色，再把每个颜色“反一下”。结果往往是局部刺眼、对比度失控，还有切换瞬间的闪烁。</p>
      <h2>先给颜色起名字</h2>
      <p>不要在组件里写具体色值。先建立一层语义变量：背景、表面、正文、次要文字、边框、强调色。</p>
      <pre><code>:root {
  --bg: #f4f7f6;
  --surface: #ffffff;
  --text: #101b18;
  --muted: #71827c;
  --border: #dce5e2;
  --accent: #0f766e;
}

[data-theme="dark"] {
  --bg: #080d0c;
  --surface: #0d1513;
  --text: #dce7e3;
  --muted: #93a8a1;
  --border: #1c2b27;
  --accent: #2dd4bf;
}</code></pre>
      <p>主题切换时，组件完全不需要知道发生了什么。它们只消费语义变量。</p>
      <h2>避免第一帧闪白</h2>
      <p>主题状态如果等 Vue 挂载后再读取，用户会先看到一帧默认主题。最简单的解法，是在 <code>&lt;head&gt;</code> 里用一小段脚本提前写入 <code>data-theme</code>。</p>
      <pre><code>const stored = localStorage.getItem('theme')
const dark = stored
  ? stored === 'dark'
  : matchMedia('(prefers-color-scheme: dark)').matches

document.documentElement.dataset.theme = dark ? 'dark' : 'light'</code></pre>
      <blockquote><p>设计系统不是限制，而是让每一次改动都发生在正确的位置。</p></blockquote>
      <h2>留一点呼吸感</h2>
      <p>青色并不需要铺满屏幕。让它出现在光标、链接、状态点和少数边线上，深色背景就有足够的层次。克制比堆叠更接近极客感。</p>
    `,
  },
  {
    slug: 'terminal-workflow',
    title: '我的终端工作流：少开窗口，多留上下文',
    description: 'tmux、fzf 和一组小脚本，如何把日常操作压缩成肌肉记忆。',
    publishedAt: '2026-10-21',
    tags: ['Linux', 'Tools', 'Workflow'],
    readingMinutes: 7,
    content: `
      <p>我理想的终端不是一排花花绿绿的窗口，而是一个上下文连续的工作台：左边写代码，右边跑测试，底下留一条随时可以接管的 shell。</p>
      <h2>一个会话，多个窗格</h2>
      <p>tmux 负责让这一切活过 SSH 断开和下班。我的日常只有三个动作：</p>
      <pre><code>tmux new -s blog
tmux a -t blog
Ctrl-b + %</code></pre>
      <p>窗格命名比窗口编号更重要。看到 <code>nvim</code>、<code>server</code>、<code>shell</code>，就知道自己现在在哪。</p>
      <h2>搜索是导航</h2>
      <p>文件、历史命令、Git 提交，全部交给模糊搜索。<code>Ctrl-r</code> 不应该只返回三个月前那条最长的命令；用 fzf 接管后，上下文会主动浮上来。</p>
      <pre><code># ~/.bashrc
eval "$(fzf --bash)"

# 历史命令
fzf-history-widget() {
  history | fzf --height 40% --reverse
}</code></pre>
      <p>工具数量不必多。关键在于每个工具都只做一件事，然后让它们自然地组合。</p>
      <h2>把重复动作写成脚本</h2>
      <ul>
        <li><code>blog</code>：进入项目并启动前后端。</li>
        <li><code>note</code>：打开当天的临时笔记。</li>
        <li><code>ship</code>：执行格式化、测试和构建。</li>
      </ul>
      <blockquote><p>自动化的意义不是让你更快，而是让你有余裕注意到更重要的问题。</p></blockquote>
      <p>最后，保持配置短小。能记住的配置，才有机会被持续维护。</p>
    `,
  },
  {
    slug: 'postgres-index-notes',
    title: 'PostgreSQL 索引没你想的那么神秘',
    description: '从执行计划出发，理解 B-tree、区分度和查询模式之间的关系。',
    publishedAt: '2026-10-02',
    tags: ['Database', 'PostgreSQL', 'Backend'],
    readingMinutes: 9,
    content: `
      <p>索引不是“加上就快”的魔法。一个没有命中查询模式的索引，只会让写入更慢、存储更贵。</p>
      <h2>先读执行计划</h2>
      <pre><code>EXPLAIN ANALYZE
SELECT id, title, published_at
FROM posts
WHERE slug = 'hello-axum';</code></pre>
      <p>如果看到 <code>Seq Scan</code>，先不要急着加索引。表有多大？过滤条件能留下多少行？返回的列是否本来就在索引里？</p>
      <h2>B-tree 喜欢什么</h2>
      <p>默认的 B-tree 适合等值、范围和有序查询。它也有自己的小脾气：多列索引遵循最左前缀，列的顺序会决定它能否被用上。</p>
      <ul>
        <li>高区分度的列放前面，除非业务查询总是按另一个列过滤。</li>
        <li>只为稳定、高频的查询模式建索引。</li>
        <li>覆盖查询可以考虑 <code>INCLUDE</code>，但别把所有列都塞进去。</li>
      </ul>
      <pre><code>CREATE INDEX idx_posts_slug
ON posts (slug);

CREATE INDEX idx_posts_published
ON posts (published_at DESC);</code></pre>
      <h2>用数据做决定</h2>
      <p>本地小表上的执行计划并不可靠。生产数据量、分布和缓存命中率，才是索引是否值得的答案。</p>
      <blockquote><p>优化不是让单条 SQL 更快，而是让整个系统在真实流量下更稳定。</p></blockquote>
      <p>每次加索引前后都跑一次 <code>EXPLAIN (ANALYZE, BUFFERS)</code>，并把结果记下来。经验就是这样一点点长出来的。</p>
    `,
  },
  {
    slug: 'clean-code-for-future-me',
    title: '写给自己的代码整洁指南',
    description: '命名、边界和删除代码：那些比格式更重要的事。',
    publishedAt: '2026-09-14',
    tags: ['Engineering', 'Notes'],
    readingMinutes: 5,
    content: `
      <p>整理代码不是为了取悦审查工具，而是为了让下一个打开文件的人少一点犹豫。那个人大概率就是三个月后的我。</p>
      <h2>名字是第一层文档</h2>
      <p>如果一个函数需要注释才能解释它在做什么，先试着换一个更准确的名字。命名会逼你面对职责是否混杂。</p>
      <pre><code>// 模糊
processData(data)

// 清楚
normalizePostTags(rawTags)</code></pre>
      <p>注释用于解释“为什么”，而不是复述“做了什么”。</p>
      <h2>让副作用待在该在的地方</h2>
      <ul>
        <li>纯计算和读写世界分开。</li>
        <li>边界层负责验证，核心逻辑保持可信输入。</li>
        <li>错误不要吞掉，翻译成调用者能理解的信息。</li>
      </ul>
      <h2>定期删除</h2>
      <p>死代码、过时配置、只使用一次的抽象，都会消耗阅读者的注意力。版本控制已经替你记住了它们，删掉不会失去什么。</p>
      <blockquote><p>简单不是写得少，而是没有多余的东西需要被理解。</p></blockquote>
      <p>每天留下一点比昨天更清楚的结构，已经足够。</p>
    `,
  },
  {
    slug: 'systemd-node-service',
    title: '用 systemd 把一个服务交给 Linux',
    description: '从 unit 文件到日志排查，把个人项目变成真正一直在线的服务。',
    publishedAt: '2026-08-26',
    tags: ['Linux', 'Ops', 'Systemd'],
    readingMinutes: 6,
    content: `
      <p>开发环境里用 <code>cargo run</code> 启动服务没问题，但一台长期在线的机器需要更明确的契约：崩溃后重启、开机自启、日志可追踪。</p>
      <h2>写一个最小 unit</h2>
      <pre><code>[Unit]
Description=sudo4 blog backend
After=network.target

[Service]
Type=simple
User=blog
WorkingDirectory=/srv/blog
ExecStart=/srv/blog/blog-server
Restart=on-failure
EnvironmentFile=/srv/blog/.env

[Install]
WantedBy=multi-user.target</code></pre>
      <p>先确保 <code>ExecStart</code> 使用绝对路径，再让配置从 <code>EnvironmentFile</code> 注入。把秘密文件权限收紧，不要让服务以 root 运行。</p>
      <h2>上线流程</h2>
      <pre><code>sudo systemctl daemon-reload
sudo systemctl enable --now blog
systemctl status blog
journalctl -u blog -f</code></pre>
      <p>最后一条命令比任何日志平台都更常用。服务启动失败时，先看这里，再猜别的。</p>
      <h2>备份与回滚</h2>
      <p>升级二进制之前，保留上一版并确认数据库迁移脚本可以回滚。稳定运行不是“从不出错”，而是出错之后能够快速回到已知状态。</p>
      <blockquote><p>基础设施的价值，往往体现在它没有存在感的时候。</p></blockquote>
      <p>现在这个博客还没有太多访问量，但服务托管的习惯应该从第一天开始。</p>
    `,
  },
]
