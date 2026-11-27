use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::error::RepositoryError;
use crate::domain::unit_of_work::UnitOfWorkFactory;

#[derive(Debug, Error)]
pub enum SettingServiceError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("invalid settings: {0}")]
    Validation(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileLink {
    pub kind: String,
    pub label: String,
    pub handle: String,
    pub href: String,
}

/// 首页 hero 终端卡片的展示内容。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TerminalHomeCard {
    pub title: String,
    pub shell: String,
    pub whoami_name: String,
    pub whoami_role: String,
    pub footer_left: String,
    pub footer_right: String,
}

impl Default for TerminalHomeCard {
    fn default() -> Self {
        Self {
            title: "visitor@sudo4: ~".to_string(),
            shell: "zsh".to_string(),
            whoami_name: "Interesting0-o".to_string(),
            whoami_role: "backend / infra / writing".to_string(),
            footer_left: "UTF-8".to_string(),
            footer_right: "git:main".to_string(),
        }
    }
}

/// 关于页 identity 终端卡片的展示内容。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TerminalAboutCard {
    pub title: String,
    pub whoami_name: String,
    pub whoami_role: String,
    pub focus: String,
    pub currently: String,
    pub status: String,
    pub since: String,
}

impl Default for TerminalAboutCard {
    fn default() -> Self {
        Self {
            title: "identity.yml".to_string(),
            whoami_name: "Interesting0-o".to_string(),
            whoami_role: "backend / infra / writing".to_string(),
            focus: "backend & infra".to_string(),
            currently: "building this blog".to_string(),
            status: "online".to_string(),
            since: "2026".to_string(),
        }
    }
}

/// 首页侧边栏 PROFILE 卡片的展示内容。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct HomeProfileCard {
    pub username: String,
    pub role: String,
    pub bio: String,
}

impl Default for HomeProfileCard {
    fn default() -> Self {
        Self {
            username: "Interesting0-o".to_string(),
            role: "builder / writer".to_string(),
            bio: "后端工程师。喜欢把系统拆开、修好，再用文字记录整个过程。".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeNowItem {
    pub key: String,
    pub value: String,
}

/// 首页侧边栏 NOW 卡片的条目列表。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct HomeNowCard {
    pub items: Vec<HomeNowItem>,
}

impl Default for HomeNowCard {
    fn default() -> Self {
        Self {
            items: vec![
                HomeNowItem {
                    key: "building".to_string(),
                    value: "MyBlogProject".to_string(),
                },
                HomeNowItem {
                    key: "reading".to_string(),
                    value: "DDIA".to_string(),
                },
            ],
        }
    }
}

/// 关于页 01 / NOW 区块。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AboutNowCard {
    pub heading: String,
    pub description: String,
    pub items: Vec<HomeNowItem>,
}

impl Default for AboutNowCard {
    fn default() -> Self {
        Self {
            heading: "现在".to_string(),
            description: "正在搭建这个博客的前后端，也在重新整理自己的知识库。比起追逐新的工具，更想把已经在用的东西理解得再深一点。".to_string(),
            items: vec![
                HomeNowItem {
                    key: "BUILDING".to_string(),
                    value: "MyBlogProject — Rust · Axum · SQLite".to_string(),
                },
                HomeNowItem {
                    key: "LEARNING".to_string(),
                    value: "数据库内部机制与分布式系统设计".to_string(),
                },
                HomeNowItem {
                    key: "WRITING".to_string(),
                    value: "工程笔记、系统实践与一些长期问题".to_string(),
                },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutPrincipleItem {
    pub title: String,
    pub description: String,
}

/// 关于页 02 / PRINCIPLES 区块。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AboutPrinciplesCard {
    pub heading: String,
    pub items: Vec<AboutPrincipleItem>,
}

impl Default for AboutPrinciplesCard {
    fn default() -> Self {
        Self {
            heading: "我相信的事".to_string(),
            items: vec![
                AboutPrincipleItem {
                    title: "简单优先".to_string(),
                    description: "能删掉的复杂度就不要留下。清晰的结构本身就是一种性能。"
                        .to_string(),
                },
                AboutPrincipleItem {
                    title: "记录过程".to_string(),
                    description: "结果会被忘记，过程中的判断和取舍才是可以复用的资产。".to_string(),
                },
                AboutPrincipleItem {
                    title: "保持好奇".to_string(),
                    description: "遇到不理解的东西，先停下来弄清楚它为什么存在。".to_string(),
                },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutStackGroup {
    pub label: String,
    pub tags: Vec<String>,
}

/// 关于页 03 / STACK 区块。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AboutStackCard {
    pub heading: String,
    pub groups: Vec<AboutStackGroup>,
}

impl Default for AboutStackCard {
    fn default() -> Self {
        Self {
            heading: "常用工具".to_string(),
            groups: vec![
                AboutStackGroup {
                    label: "LANGUAGES".to_string(),
                    tags: vec![
                        "Rust".to_string(),
                        "TypeScript".to_string(),
                        "SQL".to_string(),
                        "Bash".to_string(),
                    ],
                },
                AboutStackGroup {
                    label: "SYSTEMS".to_string(),
                    tags: vec![
                        "Linux".to_string(),
                        "PostgreSQL".to_string(),
                        "Redis".to_string(),
                        "Docker".to_string(),
                    ],
                },
                AboutStackGroup {
                    label: "DAILY".to_string(),
                    tags: vec![
                        "Neovim".to_string(),
                        "tmux".to_string(),
                        "Git".to_string(),
                        "fzf".to_string(),
                    ],
                },
            ],
        }
    }
}

/// 关于页 04 / THIS SITE 区块。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AboutSiteCard {
    pub heading: String,
    pub description: String,
    pub quote: String,
}

impl Default for AboutSiteCard {
    fn default() -> Self {
        Self {
            heading: "关于这个站点".to_string(),
            description: "这是一个极简的个人空间，用 Vue 3、TypeScript 和 Vite 构建，文章通过 Rust/Axum API 从 SQLite 读取。界面保持克制，把注意力留给文字。".to_string(),
            quote: "先让它跑起来，再让它变好。".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteSettings {
    pub site_title: String,
    pub site_tagline: String,
    pub profile_name: String,
    pub profile_handle: String,
    pub profile_tagline: String,
    pub profile_bio: String,
    #[serde(default = "default_links")]
    pub profile_links: Vec<ProfileLink>,
    pub about_markdown: String,
    #[serde(default)]
    pub terminal_home: TerminalHomeCard,
    #[serde(default)]
    pub terminal_about: TerminalAboutCard,
    #[serde(default)]
    pub home_profile: HomeProfileCard,
    #[serde(default)]
    pub home_now: HomeNowCard,
    #[serde(default)]
    pub about_now: AboutNowCard,
    #[serde(default)]
    pub about_principles: AboutPrinciplesCard,
    #[serde(default)]
    pub about_stack: AboutStackCard,
    #[serde(default)]
    pub about_site: AboutSiteCard,
}

fn default_links() -> Vec<ProfileLink> {
    Vec::new()
}

impl Default for SiteSettings {
    fn default() -> Self {
        Self {
            site_title: "sudo4".to_string(),
            site_tagline: "在终端之外记录".to_string(),
            profile_name: "sudo4".to_string(),
            profile_handle: "@sudo4".to_string(),
            profile_tagline: String::new(),
            profile_bio: String::new(),
            profile_links: Vec::new(),
            about_markdown: String::new(),
            terminal_home: TerminalHomeCard::default(),
            terminal_about: TerminalAboutCard::default(),
            home_profile: HomeProfileCard::default(),
            home_now: HomeNowCard::default(),
            about_now: AboutNowCard::default(),
            about_principles: AboutPrinciplesCard::default(),
            about_stack: AboutStackCard::default(),
            about_site: AboutSiteCard::default(),
        }
    }
}

pub async fn get(factory: &dyn UnitOfWorkFactory) -> Result<SiteSettings, SettingServiceError> {
    let mut unit_of_work = factory.begin().await?;
    let pairs = unit_of_work.settings().get_all().await?;
    unit_of_work.commit().await?;
    Ok(build(pairs))
}

pub async fn update(
    factory: &dyn UnitOfWorkFactory,
    settings: &SiteSettings,
) -> Result<SiteSettings, SettingServiceError> {
    validate(settings)?;

    let mut unit_of_work = factory.begin().await?;

    {
        let mut repository = unit_of_work.settings();

        repository
            .set("site_title", settings.site_title.trim())
            .await?;
        repository
            .set("site_tagline", settings.site_tagline.trim())
            .await?;
        repository
            .set("profile_name", settings.profile_name.trim())
            .await?;
        repository
            .set("profile_handle", settings.profile_handle.trim())
            .await?;
        repository
            .set("profile_tagline", settings.profile_tagline.trim())
            .await?;
        repository
            .set("profile_bio", settings.profile_bio.trim())
            .await?;
        repository
            .set(
                "profile_links",
                &serde_json::to_string(&settings.profile_links)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set("about_markdown", &settings.about_markdown)
            .await?;
        repository
            .set(
                "terminal_home",
                &serde_json::to_string(&settings.terminal_home)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set(
                "terminal_about",
                &serde_json::to_string(&settings.terminal_about)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set(
                "home_profile",
                &serde_json::to_string(&settings.home_profile)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set(
                "home_now",
                &serde_json::to_string(&settings.home_now)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set(
                "about_now",
                &serde_json::to_string(&settings.about_now)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set(
                "about_principles",
                &serde_json::to_string(&settings.about_principles)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set(
                "about_stack",
                &serde_json::to_string(&settings.about_stack)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
        repository
            .set(
                "about_site",
                &serde_json::to_string(&settings.about_site)
                    .map_err(|error| SettingServiceError::Validation(error.to_string()))?,
            )
            .await?;
    }

    unit_of_work.commit().await?;

    get(factory).await
}

fn build(pairs: Vec<(String, String)>) -> SiteSettings {
    let mut settings = SiteSettings::default();

    for (key, value) in pairs {
        match key.as_str() {
            "site_title" if !value.is_empty() => settings.site_title = value,
            "site_tagline" => settings.site_tagline = value,
            "profile_name" if !value.is_empty() => settings.profile_name = value,
            "profile_handle" => settings.profile_handle = value,
            "profile_tagline" => settings.profile_tagline = value,
            "profile_bio" => settings.profile_bio = value,
            "profile_links" => {
                if let Ok(links) = serde_json::from_str::<Vec<ProfileLink>>(&value) {
                    settings.profile_links = links;
                }
            }
            "about_markdown" => settings.about_markdown = value,
            "terminal_home" => {
                if let Ok(card) = serde_json::from_str::<TerminalHomeCard>(&value) {
                    settings.terminal_home = card;
                }
            }
            "terminal_about" => {
                if let Ok(card) = serde_json::from_str::<TerminalAboutCard>(&value) {
                    settings.terminal_about = card;
                }
            }
            "home_profile" => {
                if let Ok(card) = serde_json::from_str::<HomeProfileCard>(&value) {
                    settings.home_profile = card;
                }
            }
            "home_now" => {
                if let Ok(card) = serde_json::from_str::<HomeNowCard>(&value) {
                    settings.home_now = card;
                }
            }
            "about_now" => {
                if let Ok(card) = serde_json::from_str::<AboutNowCard>(&value) {
                    settings.about_now = card;
                }
            }
            "about_principles" => {
                if let Ok(card) = serde_json::from_str::<AboutPrinciplesCard>(&value) {
                    settings.about_principles = card;
                }
            }
            "about_stack" => {
                if let Ok(card) = serde_json::from_str::<AboutStackCard>(&value) {
                    settings.about_stack = card;
                }
            }
            "about_site" => {
                if let Ok(card) = serde_json::from_str::<AboutSiteCard>(&value) {
                    settings.about_site = card;
                }
            }
            _ => {}
        }
    }

    settings
}

fn validate(settings: &SiteSettings) -> Result<(), SettingServiceError> {
    if settings.site_title.trim().is_empty() {
        return Err(SettingServiceError::Validation(
            "site_title is required".to_string(),
        ));
    }

    if settings.profile_name.trim().is_empty() {
        return Err(SettingServiceError::Validation(
            "profile_name is required".to_string(),
        ));
    }

    if settings.profile_links.len() > 12 {
        return Err(SettingServiceError::Validation(
            "profile_links must contain at most 12 items".to_string(),
        ));
    }

    for link in &settings.profile_links {
        if link.label.trim().is_empty() || link.href.trim().is_empty() {
            return Err(SettingServiceError::Validation(
                "profile_links items need label and href".to_string(),
            ));
        }
    }

    if settings.about_markdown.chars().count() > 20_000 {
        return Err(SettingServiceError::Validation(
            "about_markdown is too long".to_string(),
        ));
    }

    for field in [
        &settings.terminal_home.title,
        &settings.terminal_home.shell,
        &settings.terminal_home.whoami_name,
        &settings.terminal_home.whoami_role,
        &settings.terminal_home.footer_left,
        &settings.terminal_home.footer_right,
        &settings.terminal_about.title,
        &settings.terminal_about.whoami_name,
        &settings.terminal_about.whoami_role,
        &settings.terminal_about.focus,
        &settings.terminal_about.currently,
        &settings.terminal_about.status,
        &settings.terminal_about.since,
        &settings.home_profile.username,
        &settings.home_profile.role,
        &settings.home_profile.bio,
    ] {
        if field.chars().count() > 120 {
            return Err(SettingServiceError::Validation(
                "terminal card fields must be at most 120 characters".to_string(),
            ));
        }
    }

    if settings.home_now.items.len() > 8 {
        return Err(SettingServiceError::Validation(
            "home_now.items must contain at most 8 entries".to_string(),
        ));
    }

    for item in &settings.home_now.items {
        if item.key.chars().count() > 80 || item.value.chars().count() > 120 {
            return Err(SettingServiceError::Validation(
                "home_now item fields are too long".to_string(),
            ));
        }
    }

    if settings.about_now.items.len() > 10 {
        return Err(SettingServiceError::Validation(
            "about_now.items must contain at most 10 entries".to_string(),
        ));
    }

    if settings.about_principles.items.len() > 10 {
        return Err(SettingServiceError::Validation(
            "about_principles.items must contain at most 10 entries".to_string(),
        ));
    }

    if settings.about_stack.groups.len() > 6 {
        return Err(SettingServiceError::Validation(
            "about_stack.groups must contain at most 6 groups".to_string(),
        ));
    }

    for group in &settings.about_stack.groups {
        if group.label.chars().count() > 40 || group.tags.len() > 12 {
            return Err(SettingServiceError::Validation(
                "about_stack group fields are invalid".to_string(),
            ));
        }

        for tag in &group.tags {
            if tag.chars().count() > 40 {
                return Err(SettingServiceError::Validation(
                    "about_stack tags must be at most 40 characters".to_string(),
                ));
            }
        }
    }

    for text in [
        &settings.about_now.heading,
        &settings.about_now.description,
        &settings.about_principles.heading,
        &settings.about_site.heading,
        &settings.about_site.quote,
    ] {
        if text.chars().count() > 300 {
            return Err(SettingServiceError::Validation(
                "about card fields must be at most 300 characters".to_string(),
            ));
        }
    }

    if settings.about_now.description.chars().count() > 500
        || settings.about_site.description.chars().count() > 500
    {
        return Err(SettingServiceError::Validation(
            "about card descriptions must be at most 500 characters".to_string(),
        ));
    }

    for item in &settings.about_now.items {
        if item.key.chars().count() > 40 || item.value.chars().count() > 120 {
            return Err(SettingServiceError::Validation(
                "about_now item fields are too long".to_string(),
            ));
        }
    }

    for item in &settings.about_principles.items {
        if item.title.chars().count() > 40 || item.description.chars().count() > 200 {
            return Err(SettingServiceError::Validation(
                "about_principles item fields are too long".to_string(),
            ));
        }
    }

    Ok(())
}
