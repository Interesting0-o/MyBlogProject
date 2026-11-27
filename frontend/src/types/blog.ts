export type PostStatus = 'draft' | 'published'

export interface Post {
  slug: string
  title: string
  description: string
  publishedAt: string
  updatedAt?: string
  tags: string[]
  readingMinutes: number
  featured?: boolean
  status?: PostStatus
  views?: number
  content: string
}

export type CommentStatus = 'pending' | 'approved' | 'rejected'

export interface Comment {
  id: number
  postSlug: string
  author: string
  content: string
  createdAt: string
  status: CommentStatus
}

export interface ProfileLink {
  kind: string
  label: string
  handle: string
  href: string
}

export interface TerminalHomeCard {
  title: string
  shell: string
  whoamiName: string
  whoamiRole: string
  footerLeft: string
  footerRight: string
}

export interface TerminalAboutCard {
  title: string
  whoamiName: string
  whoamiRole: string
  focus: string
  currently: string
  status: string
  since: string
}

export interface HomeProfileCard {
  username: string
  role: string
  bio: string
}

export interface HomeNowItem {
  key: string
  value: string
}

export interface HomeNowCard {
  items: HomeNowItem[]
}

export interface AboutNowItem {
  key: string
  value: string
}

export interface AboutNowCard {
  heading: string
  description: string
  items: AboutNowItem[]
}

export interface AboutPrincipleItem {
  title: string
  description: string
}

export interface AboutPrinciplesCard {
  heading: string
  items: AboutPrincipleItem[]
}

export interface AboutStackGroup {
  label: string
  tags: string[]
}

export interface AboutStackCard {
  heading: string
  groups: AboutStackGroup[]
}

export interface AboutSiteCard {
  heading: string
  description: string
  quote: string
}

export interface SiteSettings {
  siteTitle: string
  siteTagline: string
  profileName: string
  profileHandle: string
  profileTagline: string
  profileBio: string
  profileLinks: ProfileLink[]
  aboutMarkdown: string
  terminalHome: TerminalHomeCard
  terminalAbout: TerminalAboutCard
  homeProfile: HomeProfileCard
  homeNow: HomeNowCard
  aboutNow: AboutNowCard
  aboutPrinciples: AboutPrinciplesCard
  aboutStack: AboutStackCard
  aboutSite: AboutSiteCard
}

export interface PostStat {
  slug: string
  title: string
  status: string
  views: number
  approvedComments: number
  pendingComments: number
}

export interface StatsSummary {
  publishedPosts: number
  draftPosts: number
  totalViews: number
  pendingComments: number
  approvedComments: number
  rejectedComments: number
  posts: PostStat[]
}
