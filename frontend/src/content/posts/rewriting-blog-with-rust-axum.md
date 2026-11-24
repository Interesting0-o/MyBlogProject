---
title: 用 Rust 和 Axum 重写博客后端
description: 从一个空的 Cargo 项目开始，把路由、配置和数据层一层层搭起来。
publishedAt: 2026-11-18
updatedAt: 2026-11-20
tags: Rust, Backend, Axum
readingMinutes: 8
featured: true
---

这个博客的前端已经先跑了起来，而后端还停在一行 `hello, world`。这次我决定不再从熟悉的框架开始，而是用 Rust 把整条链路重新搭一遍。

## 为什么是 Axum？

Axum 足够轻，路由、提取器和响应类型都贴着标准库与 Tower 生态。它不急着替我决定项目结构，也不会用一层又一层的隐藏状态把问题包起来。

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hello, from sudo4" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

先让最小的服务跑起来，确认端口、日志和错误处理都在预期之内，再逐步加上数据库、鉴权与缓存。顺序比速度重要。

## 先把边界画清楚

我给项目定下了三条约束：

- 配置只从环境变量进入，不散落在业务代码里。
- 数据库访问收敛到一个模块，handler 不直接拼 SQL。
- 所有对外响应都有明确的结构，错误也不例外。

这些约束不会让第一版更漂亮，但会让第二版更容易继续。

> 能用一晚上写出来的东西，也值得用一周时间想清楚它应该如何生长。

## 下一步

等文章与项目接口稳定后，我会把前端的数据层切到真实 API。到那时，现在这些死数据会变成数据库里的种子，而不是被丢掉。

构建仍然继续。终端还开着。
