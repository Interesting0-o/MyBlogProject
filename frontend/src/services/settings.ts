import { profile } from '@/content/profile'
import type { SiteSettings } from '@/types/blog'

const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL || '/api').replace(/\/+$/, '')

export const defaultSettings: SiteSettings = {
  siteTitle: 'sudo4',
  siteTagline: '在终端之外记录',
  profileName: profile.name,
  profileHandle: profile.handle,
  profileTagline: profile.tagline,
  profileBio: profile.bio,
  profileLinks: profile.links.map((link) => ({
    kind: link.kind,
    label: link.label,
    handle: link.handle,
    href: link.href,
  })),
  aboutMarkdown: '',
  terminalHome: {
    title: 'visitor@sudo4: ~',
    shell: 'zsh',
    whoamiName: 'Interesting0-o',
    whoamiRole: 'backend / infra / writing',
    footerLeft: 'UTF-8',
    footerRight: 'git:main',
  },
  terminalAbout: {
    title: 'identity.yml',
    whoamiName: 'Interesting0-o',
    whoamiRole: 'backend / infra / writing',
    focus: 'backend & infra',
    currently: 'building this blog',
    status: 'online',
    since: '2026',
  },
  homeProfile: {
    username: 'Interesting0-o',
    role: 'builder / writer',
    bio: '后端工程师。喜欢把系统拆开、修好，再用文字记录整个过程。',
  },
  homeNow: {
    items: [
      { key: 'building', value: 'MyBlogProject' },
      { key: 'reading', value: 'DDIA' },
    ],
  },
  aboutNow: {
    heading: '现在',
    description:
      '正在搭建这个博客的前后端，也在重新整理自己的知识库。比起追逐新的工具，更想把已经在用的东西理解得再深一点。',
    items: [
      { key: 'BUILDING', value: 'MyBlogProject — Rust · Axum · SQLite' },
      { key: 'LEARNING', value: '数据库内部机制与分布式系统设计' },
      { key: 'WRITING', value: '工程笔记、系统实践与一些长期问题' },
    ],
  },
  aboutPrinciples: {
    heading: '我相信的事',
    items: [
      { title: '简单优先', description: '能删掉的复杂度就不要留下。清晰的结构本身就是一种性能。' },
      { title: '记录过程', description: '结果会被忘记，过程中的判断和取舍才是可以复用的资产。' },
      { title: '保持好奇', description: '遇到不理解的东西，先停下来弄清楚它为什么存在。' },
    ],
  },
  aboutStack: {
    heading: '常用工具',
    groups: [
      { label: 'LANGUAGES', tags: ['Rust', 'TypeScript', 'SQL', 'Bash'] },
      { label: 'SYSTEMS', tags: ['Linux', 'PostgreSQL', 'Redis', 'Docker'] },
      { label: 'DAILY', tags: ['Neovim', 'tmux', 'Git', 'fzf'] },
    ],
  },
  aboutSite: {
    heading: '关于这个站点',
    description:
      '这是一个极简的个人空间，用 Vue 3、TypeScript 和 Vite 构建，文章通过 Rust/Axum API 从 SQLite 读取。界面保持克制，把注意力留给文字。',
    quote: '先让它跑起来，再让它变好。',
  },
}

let cachedSettings: SiteSettings | undefined

export async function fetchSiteSettings(): Promise<SiteSettings> {
  if (cachedSettings) return cachedSettings

  try {
    const response = await fetch(`${apiBaseUrl}/settings`, {
      headers: { Accept: 'application/json' },
    })

    if (response.ok) {
      cachedSettings = { ...defaultSettings, ...(await response.json()) }
    }
  } catch {
    // 请求失败时使用内置默认值
  }

  return cachedSettings ?? defaultSettings
}

export function invalidateSettingsCache(): void {
  cachedSettings = undefined
}
