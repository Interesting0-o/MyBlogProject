# MyBlogProject

一个使用 Vue 3 + Rust/Axum 构建的个人博客。

## 本地开发

```bash
cd frontend && yarn && yarn dev
cd backend && cargo run
```

前端环境配置位于 `frontend/.env`，可参考 `frontend/.env.example`。后端配置位于根目录 `.env`。

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

## Release 配置

前端构建可通过仓库的 Actions Variables 配置：

- `VITE_API_BASE_URL`：默认 `/api`
- `VITE_USE_MOCK_DATA`：默认 `true`

如果发布 Release 失败，请在仓库 `Settings → Actions → General → Workflow permissions` 中允许 `Read and write permissions`。
