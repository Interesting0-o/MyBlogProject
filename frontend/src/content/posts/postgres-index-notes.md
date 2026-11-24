---
title: PostgreSQL 索引没你想的那么神秘
description: 从执行计划出发，理解 B-tree、区分度和查询模式之间的关系。
publishedAt: 2026-10-02
tags: Database, PostgreSQL, Backend
readingMinutes: 9
---

索引不是“加上就快”的魔法。一个没有命中查询模式的索引，只会让写入更慢、存储更贵。

## 先读执行计划

```sql
EXPLAIN ANALYZE
SELECT id, title, published_at
FROM posts
WHERE slug = 'hello-axum';
```

如果看到 `Seq Scan`，先不要急着加索引。表有多大？过滤条件能留下多少行？返回的列是否本来就在索引里？

## B-tree 喜欢什么

默认的 B-tree 适合等值、范围和有序查询。它也有自己的小脾气：多列索引遵循最左前缀，列的顺序会决定它能否被用上。

- 高区分度的列放前面，除非业务查询总是按另一个列过滤。
- 只为稳定、高频的查询模式建索引。
- 覆盖查询可以考虑 `INCLUDE`，但别把所有列都塞进去。

```sql
CREATE INDEX idx_posts_slug
ON posts (slug);

CREATE INDEX idx_posts_published
ON posts (published_at DESC);
```

## 用数据做决定

本地小表上的执行计划并不可靠。生产数据量、分布和缓存命中率，才是索引是否值得的答案。

> 优化不是让单条 SQL 更快，而是让整个系统在真实流量下更稳定。

每次加索引前后都跑一次 `EXPLAIN (ANALYZE, BUFFERS)`，并把结果记下来。经验就是这样一点点长出来的。
