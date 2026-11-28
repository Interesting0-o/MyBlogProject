# 生产部署与自动发布

本文档是 `sudo4.me` 当前生产环境的完整部署说明。默认约定：

| 项目 | 值 |
| --- | --- |
| 服务器仓库 | `/opt/sudo4/repo` |
| 部署脚本 | `/opt/sudo4/deploy`（由服务器管理） |
| 运行目录 | `/opt/sudo4` |
| 人工管理用户 | `azureuser`（sudo 需密码） |
| CI 部署用户 | `deploy`（仅可免密执行部署脚本） |
| 后端服务用户 | `sudo4` |
| systemd 服务 | `sudo4-api` |
| 域名 | `sudo4.me`、`www.sudo4.me` |

路径、用户名和域名可以替换成实际值，但 GitHub Secrets/Variables、sudoers、systemd 和 nginx 必须保持一致。

安全模型：root 只执行 `/opt/sudo4/deploy/` 下由 root 拥有的脚本；`deploy` 用户可拉代码、可触发部署，但不能修改 root 执行的脚本。

## 1. 整体流程

```text
本地 push main
   ↓
release.yml：构建前端 + 后端，打包并发布 GitHub Release(latest)
   ↓
deploy.yml：Release 构建成功后以 deploy 用户 SSH 登录服务器
   ↓
服务器：cd /opt/sudo4/repo → git pull --ff-only
        → sudo -n /opt/sudo4/deploy/deploy.sh latest
   ↓
deploy.sh：下载 Release → 备份 SQLite → 解压新版本 → 切换 current 软链
           → 重启 sudo4-api → 检查 /health → 失败自动回滚
   ↓
nginx：/opt/sudo4/current/dist 提供前端，/api 和 /health 反代 127.0.0.1:3000
```

## 2. 服务器一次性初始化

### 2.1 创建用户

| 用户 | 职责 | 登录方式 | sudo 权限 |
| --- | --- | --- | --- |
| `azureuser` | 人工管理 | 个人密钥登录 | 全量，需密码 |
| `deploy` | GitHub Action 专用 | 部署专用密钥，禁用密码 | 仅 `NOPASSWD: /opt/sudo4/deploy/deploy.sh` |
| `sudo4` | 运行后端 | nologin | 无 |
| `www-data` | nginx worker | nologin | 无 |

```bash
# 服务用户
sudo useradd --system --user-group --home /opt/sudo4 --shell /usr/sbin/nologin sudo4

# CI 部署用户：禁止密码登录，且不要加入 sudo 组
sudo adduser --disabled-password --gecos "" deploy

id azureuser
id deploy
id sudo4
```

### 2.2 克隆仓库

仓库归 `deploy` 用户所有，部署时由该用户执行 `git pull`：

```bash
sudo install -d -o deploy -g deploy /opt/sudo4/repo
sudo -u deploy git clone https://github.com/Interesting0-o/MyBlogProject.git /opt/sudo4/repo
sudo -u deploy git -C /opt/sudo4/repo pull --ff-only
```

服务器不能直连 GitHub 时，给 `deploy` 用户配置代理：

```bash
sudo -u deploy git config --global https.proxy http://代理地址:端口
sudo -u deploy git config --global http.proxy  http://代理地址:端口
```

`fetch-release.sh` 在 root 下通过 `SUDO_USER` 读取触发部署的用户（通常是 `deploy`）的代理配置。

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

部署脚本由服务器管理（`/opt/sudo4/deploy/`，root:root 755），不跟随仓库更新。CI 只允许 `deploy` 用户执行部署脚本，不授予完整 sudo：

```bash
echo 'deploy ALL=(root) NOPASSWD: /opt/sudo4/deploy/deploy.sh' | sudo tee /etc/sudoers.d/deploy
sudo chmod 440 /etc/sudoers.d/deploy
sudo visudo -cf /etc/sudoers.d/deploy
sudo -l -U deploy
```

`azureuser` 只用于人工管理，sudo 需要密码；不要为它添加部署免密条目。sudoers 同一命令多条匹配时最后匹配生效，因此部署规则应保持为独立文件并避免被其他文件覆盖。

如果部署脚本路径不同，sudoers 里的路径也要同步修改。受限 sudoers 不允许 `sudo VAR=...`，所以版本号使用位置参数：

```bash
sudo /opt/sudo4/deploy/deploy.sh latest
sudo /opt/sudo4/deploy/deploy.sh v0.1.0
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

服务运行目录结构与属主：

```text
/opt/sudo4/                     root:root      755
├── deploy/                     root:root      755   ← root 唯一执行的代码来源
│   ├── deploy.sh               root:root      755
│   └── fetch-release.sh        root:root      755
├── repo/                       deploy:deploy  755   ← 仅代码来源（git pull）
├── releases/                   root:root      755   ← 仅 root 部署脚本写入
├── current -> releases/...     root
├── backups/                    root:root      700
├── resource/                   sudo4:sudo4    750
└── .env                        sudo4:sudo4    600   ← 真实密钥唯一存放点
```

`.env` 不进入 git，也不在 `repo/` 工作区留存。`DATABASE_URL` 必须指向 `/opt/sudo4/resource`，否则会被 systemd 的 `ReadWritePaths` 拦截。

`/opt/sudo4` 本身应为 `root:root 755`；部署脚本以 root 运行时会强制 `releases/` 为 `root:root 755`、`backups/` 为 `root:root 700`、`resource/` 为 `sudo4:sudo4 750`，并创建 `/opt/sudo4/.deploy.lock` 防止并发部署。

### 2.8 首次部署

```bash
sudo /opt/sudo4/deploy/deploy.sh latest
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
| `DEPLOY_USER` | CI 部署用户，固定为 `deploy` |
| `DEPLOY_KEY` | `deploy` 用户专用私钥的 base64 单行文本 |
| `DEPLOY_KNOWN_HOSTS` | 可选，服务器主机指纹 |

生成部署密钥并安装到 `deploy` 用户：

```bash
ssh-keygen -t ed25519 -C "github-actions-deploy" -f ~/.ssh/sudo4_deploy -N ""
ssh-keygen -y -P '' -f ~/.ssh/sudo4_deploy

cat ~/.ssh/sudo4_deploy.pub | ssh azureuser@sudo4.me \
  'sudo install -d -o deploy -g deploy -m 700 /home/deploy/.ssh && sudo -u deploy tee /home/deploy/.ssh/authorized_keys >/dev/null && sudo chmod 600 /home/deploy/.ssh/authorized_keys'

ssh -i ~/.ssh/sudo4_deploy deploy@sudo4.me 'id'
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

密钥轮换流程：生成新密钥对 → 安装到 `deploy` 用户 → 用新私钥验证 SSH → 覆盖 GitHub `DEPLOY_KEY` → 手动触发一次部署验收 → 从服务器 `authorized_keys` 删除旧公钥。仅覆盖 GitHub secret 不算吊销，真正的吊销是删除服务器上的公钥。

### 3.2 Variables

| 名称 | 值 |
| --- | --- |
| `DEPLOY_PATH` | `/opt/sudo4/repo`（仓库目录，不是运行目录） |
| `DEPLOY_PORT` | `22` |
| `DEPLOY_SCRIPT` | 可选，默认 `/opt/sudo4/deploy/deploy.sh` |

```bash
gh variable set DEPLOY_PATH --body "/opt/sudo4/repo"
gh variable set DEPLOY_PORT --body "22"
gh variable list
```

`DEPLOY_SCRIPT` 是服务器上的部署脚本路径；只有脚本不在默认位置时才需要设置。

## 4. 自动部署

`release.yml`：

- 推送到 `main`：构建前端和后端，发布滚动 Release `latest`
- 推送 `v*` 标签：发布版本化 Release
- 后端构建使用 `SQLX_OFFLINE=true`

`deploy.yml`：

- 监听 `Build release artifacts` 在 `main` 上构建成功
- 以 `deploy` 用户 SSH 到服务器执行：
  ```bash
  cd /opt/sudo4/repo
  git pull --ff-only
  sudo -n /opt/sudo4/deploy/deploy.sh latest
  ```
- root 只执行 `/opt/sudo4/deploy/` 下的脚本，不执行工作区代码
- 串行部署，使用 `production` environment

`deploy.sh` 的行为：

1. 通过 `/opt/sudo4/.deploy.lock` 加锁，并发部署直接失败
2. 校验 release tag、`KEEP_RELEASES`、`KEEP_BACKUPS` 等参数
3. 检查 systemd 单元是否存在
4. 从单元读取 `User`/`Group`，用于 chown `.env`、`resource/`
5. 下载 Release；`SHA256SUMS` 缺失或校验不一致都会直接失败
6. 用 `sqlite3 .backup` 备份数据库
7. 解压到 `releases/<标签-时间>/`，并强制为 `root:root`、去掉组/其他写权限
8. 原子切换 `current` 软链
9. 重启 `sudo4-api`
10. 轮询 `/health`，失败时切回上一版本
11. 清理旧版本和旧备份，默认各保留 5 和 10 个

首次部署且 `/opt/sudo4/.env` 不存在时，`.env.example` 从经过校验的 Release 产物读取，不再读取 deploy 可写的仓库目录。

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
sudo /opt/sudo4/deploy/deploy.sh v0.1.0
```

回滚：

- 健康检查失败时自动回滚
- 手动回滚就是重新部署旧版本标签

脚本变更流程：

1. 修改仓库 `scripts/deploy.sh` / `scripts/fetch-release.sh`，提交 PR 并合并
2. 服务器 `deploy` 用户拉取仓库后，由人工确认并同步到 root 目录：

   ```bash
   sudo install -o root -g root -m 755 /opt/sudo4/repo/scripts/deploy.sh /opt/sudo4/deploy/deploy.sh
   sudo install -o root -g root -m 755 /opt/sudo4/repo/scripts/fetch-release.sh /opt/sudo4/deploy/fetch-release.sh
   bash -n /opt/sudo4/deploy/deploy.sh
   bash -n /opt/sudo4/deploy/fetch-release.sh
   ```

3. 手动部署一次做验收

这一步 `sudo install` 就是“root 执行什么代码”的人工审批点；`deploy` 用户和 CI 都无法直接修改 `/opt/sudo4/deploy/`。

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
| `Deploy script not found or not executable` | `DEPLOY_SCRIPT` 路径不对，确认服务器上脚本存在且可执行 |
| `another deployment is already running` | 另一个手动/自动部署仍在执行；等它结束后重试 |
| `SHA256SUMS is unavailable` | Release 不完整或下载失败；等构建完成或重新发布 |
| `Checksum mismatch` | Release 产物被覆盖或下载损坏；重新运行部署 |
| `invalid release tag` | tag 含空格或非法字符；使用 `v0.1.0`、`latest` 这类值 |
| `Refusing unsafe destination` | `DEST` 不是绝对路径或是 `/`；检查调用参数 |
| `sudo: a password is required` | `DEPLOY_USER` 仍是旧用户，或 sudoers 仍指向 `repo/scripts`；改为 `deploy` 和 `/opt/sudo4/deploy/deploy.sh` |
| 部署脚本启动即退出 | 脚本里残留 `git rev-parse` 位置推导；`/opt/sudo4/deploy` 不在 git 仓库内 |
| root 执行 git 报 `dubious ownership` | 有脚本仍在对 `repo/` 执行 git；部署脚本应只用固定路径读 `.env.example`，git 操作交给 `deploy` 用户 |
| 健康检查 curl 失败 | 确认地址是 `http://127.0.0.1:3000/health`，不是 8080 或 `/healthz` |
| `not allowed to set ... RELEASE_TAG` | sudoers 不允许环境变量；用 `sudo /opt/sudo4/deploy/deploy.sh latest` |
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
sudo -l -U deploy
sudo -l -U azureuser
ls -l /opt/sudo4/current /opt/sudo4/resource /opt/sudo4/backups
ls -l /opt/sudo4/deploy
```
