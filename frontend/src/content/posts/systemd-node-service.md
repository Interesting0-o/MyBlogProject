---
title: 用 systemd 把一个服务交给 Linux
description: 从 unit 文件到日志排查，把个人项目变成真正一直在线的服务。
publishedAt: 2026-08-26
tags: Linux, Ops, Systemd
readingMinutes: 6
---

开发环境里用 `cargo run` 启动服务没问题，但一台长期在线的机器需要更明确的契约：崩溃后重启、开机自启、日志可追踪。

## 写一个最小 unit

```ini
[Unit]
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
WantedBy=multi-user.target
```

先确保 `ExecStart` 使用绝对路径，再让配置从 `EnvironmentFile` 注入。把秘密文件权限收紧，不要让服务以 root 运行。

## 上线流程

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now blog
systemctl status blog
journalctl -u blog -f
```

最后一条命令比任何日志平台都更常用。服务启动失败时，先看这里，再猜别的。

## 备份与回滚

升级二进制之前，保留上一版并确认数据库迁移脚本可以回滚。稳定运行不是“从不出错”，而是出错之后能够快速回到已知状态。

> 基础设施的价值，往往体现在它没有存在感的时候。

现在这个博客还没有太多访问量，但服务托管的习惯应该从第一天开始。
