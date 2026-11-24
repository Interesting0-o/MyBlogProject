export type ProfileLinkKind = 'github' | 'bilibili' | 'repository' | 'email'

export interface ProfileLink {
  kind: ProfileLinkKind
  label: string
  handle: string
  href: string
}

export const profile = {
  name: 'sudo4',
  handle: '@Interesting0-o',
  role: 'backend engineer / writer',
  tagline: '保持好奇，持续构建。',
  bio: '关注后端与基础设施，喜欢把复杂系统拆开、理解，再用更简单的方式装回去。',
  avatar: '/avatar.jpg',
  links: [
    {
      kind: 'github',
      label: 'GitHub',
      handle: '@Interesting0-o',
      href: 'https://github.com/Interesting0-o',
    },
    {
      kind: 'bilibili',
      label: 'Bilibili',
      handle: '待配置',
      href: '',
    },
    {
      kind: 'repository',
      label: 'Repository',
      handle: 'MyBlogProject',
      href: 'https://github.com/Interesting0-o/MyBlogProject',
    },
    {
      kind: 'email',
      label: 'Email',
      handle: '2597256916@qq.com',
      href: 'mailto:2597256916@qq.com',
    },
  ] satisfies ProfileLink[],
}
