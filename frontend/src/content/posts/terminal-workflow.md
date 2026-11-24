---
title: 我的终端工作流：少开窗口，多留上下文
description: tmux、fzf 和一组小脚本，如何把日常操作压缩成肌肉记忆。
publishedAt: 2026-10-21
tags: Linux, Tools, Workflow
readingMinutes: 7
---

我理想的终端不是一排花花绿绿的窗口，而是一个上下文连续的工作台：左边写代码，右边跑测试，底下留一条随时可以接管的 shell。

## 一个会话，多个窗格

tmux 负责让这一切活过 SSH 断开和下班。我的日常只有三个动作：

```bash
tmux new -s blog
tmux a -t blog
Ctrl-b + %
```

窗格命名比窗口编号更重要。看到 `nvim`、`server`、`shell`，就知道自己现在在哪。

## 搜索是导航

文件、历史命令、Git 提交，全部交给模糊搜索。`Ctrl-r` 不应该只返回三个月前那条最长的命令；用 fzf 接管后，上下文会主动浮上来。

```bash
# ~/.bashrc
eval "$(fzf --bash)"

# 历史命令
fzf-history-widget() {
  history | fzf --height 40% --reverse
}
```

工具数量不必多。关键在于每个工具都只做一件事，然后让它们自然地组合。

## 把重复动作写成脚本

- `blog`：进入项目并启动前后端。
- `note`：打开当天的临时笔记。
- `ship`：执行格式化、测试和构建。

> 自动化的意义不是让你更快，而是让你有余裕注意到更重要的问题。

最后，保持配置短小。能记住的配置，才有机会被持续维护。
