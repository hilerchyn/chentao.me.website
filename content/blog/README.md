# 博客文章

每篇文章是一篇 Markdown，放在类型目录里。

## 加一篇文章

1. 选定类型目录，例如 `architecture/`。
2. 新建两个文件，文件名相同、语言后缀不同：

   - `my-topic.zh.md`
   - `my-topic.en.md`

3. 文件开头用 `+++` 包一层 TOML：

```toml
+++
title = "标题"
date = "2026-11-03"
summary = "列表页上的一句摘要。"
+++

正文从这里开始，支持普通 Markdown。
```

中文地址：`/blog/architecture/my-topic`  
英文地址：`/en/blog/architecture/my-topic`

只有一种语言时，另一种语言的列表里不会出现这篇。

## 加一类文章

1. 在 `types.toml` 末尾加一段：

```toml
[[type]]
id = "ops"
order = 7
name_zh = "运维笔记"
name_en = "Operations"
desc_zh = "机器怎么跑，故障怎么记。"
desc_en = "How the machines run, and how failures are written down."
```

2. 新建目录 `content/blog/ops/`。
3. 按上面的规则放入 `.zh.md` / `.en.md`。

`id` 只能用小写字母、数字和连字符，会进 URL。  
若只建了目录、还没写进 `types.toml`，列表里仍会显示该类，名称暂时等于目录名。
