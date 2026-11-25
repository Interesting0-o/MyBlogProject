# MyBlogProject

一个使用 Vue 3 + Rust/Axum 构建的个人博客。

## 本地开发

```bash
cd frontend && yarn && yarn dev
cd backend && cargo run
```

前后端共用根目录 `.env`，配置模板为 `.env.example`。`VITE_` 前缀变量会注入前端构建，其余变量仅后端读取。

后端默认使用 SQLite，数据库位于 `resource/sudo4.db`，首次启动会自动创建目录并执行迁移。

前端默认通过 `/api/posts` 读取后端文章；设置 `VITE_USE_MOCK_DATA=true` 可以切回内置 mock，方便单独开发前端。开发环境由 Vite 代理 `/api`，部署时需要把 `/api` 反向代理到后端。

SQL 使用 SQLx 查询宏做编译期校验，离线元数据在 `backend/.sqlx/`。修改 SQL 后需要在 `backend/` 下执行 `DATABASE_URL=sqlite://../resource/sudo4.db cargo sqlx prepare` 并提交更新后的 `.sqlx/`。

后端请求日志默认输出 `sudo4_api=info`，可用 `RUST_LOG=debug cargo run` 调整级别。

## 写文章

文章使用 Markdown 和 Frontmatter，文件名作为文章 URL 的 slug。`frontend/src/content/posts/` 目前保留用于 mock 模式和文章导入，已发布内容的源头是 SQLite：

```markdown
---
title: 文章标题
description: 文章摘要
publishedAt: 2026-11-24
updatedAt: 2026-11-25
tags: Rust, Backend
readingMinutes: 6
featured: true
---

这里开始写正文。
```

现有文章已通过 `backend/migrations/0002_seed_posts.sql` 写入 SQLite，正文以 Markdown 存储，前端统一用 `marked` 渲染。`updatedAt` 和 `featured` 可省略；`featured: true` 会启用首页的精选样式。

发布页位于 `/#/admin`。当前可以撰写、预览、复制或下载 Markdown，但认证和发布 API 尚未接入，发布按钮保持禁用；在此之前，新增或修改文章需要更新 SQLite 的 `posts` 表。后续计划见 `TODO.md`。

关于页的个人资料在 `frontend/src/content/profile.ts` 中维护，头像文件为 `frontend/public/avatar.jpg`，替换文件即可更新。

## 自动构建产物

推送 `main` 分支会触发 `.github/workflows/release.yml`，构建并更新 `latest` Release：

- `sudo4-linux-x86_64.tar.gz`：Linux x86_64 后端二进制、前端静态文件和 `.env.example`
- `SHA256SUMS`：下载校验文件

推送 `v*` 标签会额外生成版本化 Release，例如 `v0.1.0`。

### 拉取后自动获取

Git 本身不会在 `clone` / `pull` 时运行构建或下载附件，需要先执行一次：

```bash
./scripts/install-hooks.sh
```

它会安装 `post-merge` Git Hook，并立即下载最新产物。之后每次 `git pull` 都会自动更新：

```text
.artifacts/
├── sudo4-api
├── dist/
└── .env.example
```

注意：Hook 下载的是最近一次成功发布的 Release；如果刚推送提交，需等 GitHub Actions 构建完成后再拉取。

### 手动下载

```bash
./scripts/fetch-release.sh
./scripts/fetch-release.sh v0.1.0
```

产物目录在 `.artifacts/`，可通过 `DEST` 修改，通过 `RELEASE_TAG` 指定版本。

## 生产部署（热更新）

生产环境建议把仓库和运行目录分开，例如把仓库放在 `/srv/MyBlogProject`，运行目录为 `/opt/sudo4`：

```text
/opt/sudo4/
├── current -> releases/v0.1.0-20261125010203/
├── releases/
├── backups/
├── resource/sudo4.db
└── .env
```

首次部署：

```bash
sudo useradd --system --user-group --home /opt/sudo4 --shell /usr/sbin/nologin sudo4
sudo install -d -o sudo4 -g sudo4 /opt/sudo4
git clone https://github.com/Interesting0-o/MyBlogProject /srv/MyBlogProject
cd /srv/MyBlogProject

sudo cp deploy/sudo4-api.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo cp deploy/nginx.conf /etc/nginx/conf.d/sudo4.conf
sudo nginx -t && sudo systemctl reload nginx

sudo ./scripts/deploy.sh
sudo systemctl enable sudo4-api
```

`deploy.sh` 会依次：下载 Release 产物 → 备份 SQLite → 解包到 `releases/<id>/` → 原子切换 `current` 软链 → 重启服务 → 健康检查 `/health`，失败时自动切回上一个版本。首次运行会自动从 `.env.example` 生成 `/opt/sudo4/.env`，上线前请检查其中的 `BACKEND_HOST`、`DATABASE_URL` 等配置。

常用操作：

```bash
sudo RELEASE_TAG=v0.1.0 ./scripts/deploy.sh   # 固定版本
sudo APP_ROOT=/srv/sudo4 ./scripts/deploy.sh  # 自定义运行目录
sudo SKIP_RESTART=1 ./scripts/deploy.sh       # 只拉取并切换，不重启
```

热更新语义：

- 文章内容存于 SQLite，通过 API 读取，改内容不需要重新构建或重启。
- 前端是静态文件，切换 `current` 后 nginx 立即读取新 `dist/`，无需 reload。
- 后端需要重启进程，存在秒级中断；要完全零停机需要 blue-green 或多实例加 upstream 切换。

生产环境建议固定 `v*` 标签，而不是使用每次 `main` 推送都会覆盖的 `latest`，也不要依赖开发用的 `post-merge` Hook 自动更新。`KEEP_RELEASES` 和 `KEEP_BACKUPS` 分别控制保留的版本与备份数量（默认 5 和 10）；`DB_PATH`、`HEALTH_URL`、`HEALTH_ATTEMPTS`、`APP_USER` 可按需覆盖。systemd 和 nginx 配置中的 `/opt/sudo4`、`sudo4` 用户、域名需要按实际环境调整。

## Release 配置

前端构建可通过仓库的 Actions Variables 配置：

- `VITE_API_BASE_URL`：默认 `/api`
- `VITE_USE_MOCK_DATA`：默认 `false`

如果发布 Release 失败，请在仓库 `Settings → Actions → General → Workflow permissions` 中允许 `Read and write permissions`。
