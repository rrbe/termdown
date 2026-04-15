# 😀 Emoji Heading Smoke Test

## ✨ 标题里的 emoji 和中文混排

这是一份专门用于验证标题图片渲染的测试文档。

### 🚀 Launch builders, not just code

- 单个 emoji: 😀 😎 ✨ 🚀
- 中英混排: Hello 世界 🌍
- 符号混排: ✅ Done · ⚠ Warning · ❌ Failed
- 常见 emoji 变体: ☀️ ❤️ ⭐️

## 🧪 Body Text Baseline

正文仍然由终端自身字体渲染，所以这里主要用来对比标题和正文的表现差异。

> 引用块里也放几个字符：💡 🛠 📦

## 📊 Table

| Case | Example |
| ---- | ------- |
| Single emoji | 😀 |
| Mixed text | 修正版 ✨ version 2 |
| Symbol-like | ✅ ⚠ ❌ |
| Variation selector | ☀️ ❤️ |

## 👨‍👩‍👧‍👦 Complex Sequence Boundary

这一行用于观察复杂 ZWJ emoji 的边界表现：👨‍👩‍👧‍👦 👩🏽‍💻 🧑‍🚀

## 🎯 Expected Result

如果修复生效，H1-H3 里的大部分单 emoji 和常见符号不应再显示成缺字框。
