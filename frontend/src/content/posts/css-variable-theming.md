---
title: 用 CSS 变量做一套不闪眼的双主题
description: 颜色不只是两套色值。先定义语义，再让光照进来。
publishedAt: 2026-11-06
tags: CSS, Design, Frontend
readingMinutes: 6
---

双主题最容易犯的错误，是先写一套浅色，再把每个颜色“反一下”。结果往往是局部刺眼、对比度失控，还有切换瞬间的闪烁。

## 先给颜色起名字

不要在组件里写具体色值。先建立一层语义变量：背景、表面、正文、次要文字、边框、强调色。

```css
:root {
  --bg: #f4f7f6;
  --surface: #ffffff;
  --text: #101b18;
  --muted: #71827c;
  --border: #dce5e2;
  --accent: #0f766e;
}

[data-theme="dark"] {
  --bg: #080d0c;
  --surface: #0d1513;
  --text: #dce7e3;
  --muted: #93a8a1;
  --border: #1c2b27;
  --accent: #2dd4bf;
}
```

主题切换时，组件完全不需要知道发生了什么。它们只消费语义变量。

## 避免第一帧闪白

主题状态如果等 Vue 挂载后再读取，用户会先看到一帧默认主题。最简单的解法，是在 `<head>` 里用一小段脚本提前写入 `data-theme`。

```js
const stored = localStorage.getItem('theme')
const dark = stored
  ? stored === 'dark'
  : matchMedia('(prefers-color-scheme: dark)').matches

document.documentElement.dataset.theme = dark ? 'dark' : 'light'
```

> 设计系统不是限制，而是让每一次改动都发生在正确的位置。

## 留一点呼吸感

青色并不需要铺满屏幕。让它出现在光标、链接、状态点和少数边线上，深色背景就有足够的层次。克制比堆叠更接近极客感。
