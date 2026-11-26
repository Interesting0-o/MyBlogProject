# 生产部署与自动发布

本文档是 `sudo4.me` 当前生产环境的完整部署说明。默认约定：

| 项目 | 值 |
| --- | --- |
| 服务器仓库 | `/opt/sudo4/repo` |
| 运行目录 | `/opt/sudo4` |
| SSH 部署用户 | `azureuser` |
| 后端服务用户 | `sudo4` |
| systemd 服务 | `sudo4-api` |
| 域名 | `sudo4.me`、`www.sudo4.me` |

路径、用户名和域名可以替换成实际值，但 GitHub Variables、sudoers、systemd 和 nginx 必须保持一致。

## 1. 整体流程

```text
本地 push main
   ↓
release.yml：构建前端 + 后端，打包并发布 GitHub Release(latest)
   ↓
deploy.yml：Release 构建成功后 SSH 登录服务器
   ↓
服务器：cd DEPLOY_PATH → git pull --ff-only → sudo ./scripts/deploy.sh latest
   ↓
deploy.sh：下载 Release → 备份 SQLite → 解压新版本 → 切换 current 软链
           → 重启 sudo4-api → 检查 /health → 失败自动回滚
   ↓
nginx：/opt/sudo4/current/dist 提供前端，/api 和 /health 反代 127.0.0.1:3000
```

## 2. 服务器一次性初始化

### 2.1 创建用户

部署用户负责 SSH、拉代码和触发部署；服务用户只负责运行后端。

```bash
# 服务用户
sudo useradd --system --user-group --home /opt/sudo4 --shell /usr/sbin/nologin sudo4

# 部署用户已存在时不需要创建
id azureuser
id sudo4
```

### 2.2 克隆仓库

```bash
sudo install -d -o azureuser -g azureuser /opt/sudo4/repo
git clone https://github.com/Interesting0-o/MyBlogProject.git /opt/sudo4/repo
cd /opt/sudo4/repo
git pull --ff-only
```

服务器不能直连 GitHub 时，先给部署用户配置代理：

```bash
git config --global https.proxy http://代理地址:端口
git config --global http.proxy  http://代理地址:端口
```

`fetch-release.sh` 在 `sudo` 下会读取部署用户的代理配置。

### 2.3 安装依赖

```bash
sudo apt update
sudo apt install -y nginx curl sqlite3 certbot python3-certbot-nginx
sudo systemctl enable --now nginx
```

`sqlite3` 用于在线备份；没有它时部署脚本会退化为直接复制数据库文件。

### 2.4 systemd

```bash
sudo cp /opt/sudo4/repo/deploy/sudo4-api.service /etc/systemd/system/
sudo systemctl daemon-reload
```

服务单元默认 `User=sudo4`、`Group=sudo4`、`WorkingDirectory=/opt/sudo4`、`ExecStart=/opt/sudo4/current/sudo4-api`。如果改用其他服务用户，修改单元里的 `User`/`Group` 后重新 `daemon-reload`；部署脚本会自动从单元读取实际用户。

### 2.5 nginx 与 HTTPS

```bash
sudo cp /opt/sudo4/repo/deploy/nginx.conf /etc/nginx/conf.d/sudo4.conf
sudo sed -i 's/server_name .*/server_name sudo4.me www.sudo4.me;/' /etc/nginx/conf.d/sudo4.conf

# Ubuntu 默认站点会抢占 server_name，先禁用
sudo rm -f /etc/nginx/sites-enabled/default

sudo nginx -t && sudo systemctl reload nginx
```

首次配置 HTTPS：

```bash
sudo certbot --nginx -d sudo4.me -d www.sudo4.me
```

如果证书已经存在、只是配置被覆盖，可以重新安装到当前 server 块：

```bash
sudo certbot --nginx --reinstall -d sudo4.me -d www.sudo4.me
```

注意：Certbot 会修改 `/etc/nginx/conf.d/sudo4.conf` 并加入 443、证书路径和 HTTP 跳转。**不要再用仓库里的 `deploy/nginx.conf` 直接覆盖它**；如果覆盖了，需要重新执行 `certbot --nginx --reinstall`。

### 2.6 免密 sudo

CI 只允许执行部署脚本，不授予完整 sudo：

```bash
echo 'azureuser ALL=(root) NOPASSWD: /opt/sudo4/repo/scripts/deploy.sh' | sudo tee /etc/sudoers.d/sudo4-deploy
sudo chmod 440 /etc/sudoers.d/sudo4-deploy
sudo visudo -cf /etc/sudoers.d/sudo4-deploy
sudo -n -l /opt/sudo4/repo/scripts/deploy.sh
```

如果仓库路径不同，sudoers 里的路径也要同步修改。受限 sudoers 不允许 `sudo VAR=...`，所以版本号使用位置参数：

```bash
sudo ./scripts/deploy.sh latest
sudo ./scripts/deploy.sh v0.1.0
```

### 2.7 运行配置和目录

首次部署会自动生成 `/opt/sudo4/.env`，也可以在部署前手动创建：

```bash
sudo cp /opt/sudo4/repo/.env.example /opt/sudo4/.env
sudo chown sudo4:sudo4 /opt/sudo4/.env
sudo chmod 600 /opt/sudo4/.env
```

默认内容：

```env
BACKEND_HOST=127.0.0.1
BACKEND_PORT=3000
DATABASE_URL=sqlite://resource/sudo4.db
VITE_API_BASE_URL=/api
```

服务运行目录结构：

```text
/opt/sudo4/
├── current -> releases/latest-20261125055312/
├── releases/
├── backups/
├── resource/sudo4.db
└── .env
```

`DATABASE_URL` 必须指向 `/opt/sudo4/resource`，否则会被 systemd 的 `ReadWritePaths` 拦截。

### 2.8 首次部署

```bash
cd /opt/sudo4/repo
sudo ./scripts/deploy.sh latest
sudo systemctl enable sudo4-api
```

验证：

```bash
sudo systemctl status sudo4-api --no-pager
curl http://127.0.0.1:3000/health
curl -I -H 'Host: sudo4.me' http://127.0.0.1/
```

## 3. GitHub 配置

### 3.1 Secrets

| 名称 | 说明 |
| --- | --- |
| `DEPLOY_HOST` | 服务器地址 |
| `DEPLOY_USER` | SSH 部署用户，例如 `azureuser` |
| `DEPLOY_KEY` | 部署私钥的 base64 单行文本 |
| `DEPLOY_KNOWN_HOSTS` | 可选，服务器主机指纹 |

生成并安装部署密钥：

```bash
ssh-keygen -t ed25519 -C "github-actions-deploy" -f ~/.ssh/sudo4_deploy -N ""
ssh-keygen -y -P '' -f ~/.ssh/sudo4_deploy
ssh-copy-id -i ~/.ssh/sudo4_deploy.pub azureuser@sudo4.me
```

写入 base64 私钥：

```bash
base64 -w0 ~/.ssh/sudo4_deploy > /tmp/deploy_key.b64
gh secret set DEPLOY_KEY < /tmp/deploy_key.b64
rm -f /tmp/deploy_key.b64
```

可选的主机指纹：

```bash
ssh-keyscan -H sudo4.me | gh secret set DEPLOY_KNOWN_HOSTS
```

Workflow 使用了 `environment: production`。**Environment secret 会覆盖 Repository secret**，所以设置时注意作用域：

```bash
gh secret list
gh secret list --env production
```

### 3.2 Variables

| 名称 | 值 |
| --- | --- |
| `DEPLOY_PATH` | `/opt/sudo4/repo` |
| `DEPLOY_PORT` | `22` |

```bash
gh variable set DEPLOY_PATH --body "/opt/sudo4/repo"
gh variable set DEPLOY_PORT --body "22"
gh variable list
```

`DEPLOY_PATH` 是仓库目录，不是运行目录 `/opt/sudo4`。

## 4. 自动部署

`release.yml`：

- 推送到 `main`：构建前端和后端，发布滚动 Release `latest`
- 推送 `v*` 标签：发布版本化 Release
- 后端构建使用 `SQLX_OFFLINE=true`

`deploy.yml`：

- 监听 `Build release artifacts` 在 `main` 上构建成功
- SSH 到服务器执行：
  ```bash
  cd /opt/sudo4/repo
  git pull --ff-only
  sudo -n ./scripts/deploy.sh latest
  ```
- 串行部署，使用 `production` environment

`deploy.sh` 的行为：

1. 检查 systemd 单元是否存在
2. 从单元读取 `User`/`Group`，用于 chown `.env`、`resource/`、`backups/`
3. 下载 Release 并校验 `SHA256SUMS`
4. 用 `sqlite3 .backup` 备份数据库
5. 解压到 `releases/<标签-时间>/`
6. 原子切换 `current` 软链
7. 重启 `sudo4-api`
8. 轮询 `/health`，失败时切回上一版本
9. 清理旧版本和旧备份，默认各保留 5 和 10 个

前端切换软链后立即生效；后端重启有几秒中断。

## 5. 日常操作

推送 main 自动部署：

```bash
git push origin main
```

版本发布：

```bash
git tag v0.1.0
git push origin v0.1.0
```

版本标签不会自动部署，构建完成后手动运行：

```bash
gh workflow run "Deploy production" -f tag=v0.1.0
```

或在服务器手动部署：

```bash
cd /opt/sudo4/repo
git pull --ff-only
sudo ./scripts/deploy.sh v0.1.0
```

回滚：

- 健康检查失败时自动回滚
- 手动回滚就是重新部署旧版本标签

查看状态：

```bash
sudo systemctl status sudo4-api --no-pager
sudo journalctl -u sudo4-api -n 50 --no-pager
ls -l /opt/sudo4/current /opt/sudo4/releases
curl -fsS https://sudo4.me/health
```

## 6. 数据与备份

- 数据库：`/opt/sudo4/resource/sudo4.db`
- 备份目录：`/opt/sudo4/backups/`
- 文章内容存于 SQLite，通过 API 读取；改文章不需要重新构建或重启
- 后端启动时自动执行迁移

手动备份：

```bash
sudo sqlite3 /opt/sudo4/resource/sudo4.db ".backup '/opt/sudo4/backups/manual-$(date -u +%Y%m%d%H%M%S).db'"
```

## 7. 排障

| 现象 | 原因与处理 |
| --- | --- |
| `Load key ... error in libcrypto` | `DEPLOY_KEY` 不是有效私钥；用 `gh secret set` 写入 base64 私钥 |
| `DEPLOY_KEY is not valid base64` | secret 不是标准 base64，或 Environment secret 覆盖了 Repository secret |
| `cd ... No such file or directory` | `DEPLOY_PATH` 不对，确认服务器仓库路径 |
| `not allowed to set ... RELEASE_TAG` | sudoers 不允许环境变量；用 `sudo ./scripts/deploy.sh latest` |
| `user sudo4 does not exist` | 创建服务用户，或修改单元 `User` 并 `daemon-reload` |
| `Read-only file system` | `DATABASE_URL` 不在 `/opt/sudo4/resource`，或目录权限不对 |
| GitHub 重跑仍用旧逻辑 | 不要点 Re-run，使用 Run workflow 从最新 `main` 发起 |
| 访问域名显示 nginx 欢迎页 | `server_name` 不匹配，或 `/etc/nginx/sites-enabled/default` 抢占 |
| `conflicting server name` | 多个 server 块声明同一域名；禁用 default 或删除重复配置 |
| HTTPS 证书失效 | 重新执行 `certbot --nginx --reinstall -d sudo4.me -d www.sudo4.me` |
| 部署后 `/health` 失败 | 查看 `journalctl -u sudo4-api -n 80 --no-pager` |

## 8. 最终检查清单

```bash
curl -I https://sudo4.me/
curl -fsS https://sudo4.me/health
curl -fsS https://sudo4.me/api/posts | head -c 200
sudo systemctl status sudo4-api --no-pager
sudo -n -l /opt/sudo4/repo/scripts/deploy.sh
ls -l /opt/sudo4/current /opt/sudo4/resource /opt/sudo4/backups
```
