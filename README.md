# timestamp_zone

[![CI](https://github.com/Tinkora/timestamp_zone/actions/workflows/test.yml/badge.svg)](https://github.com/Tinkora/timestamp_zone/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

浏览器原生的 Unix 时间戳 ↔ ISO 8601 ↔ 多时区转换工具。所有计算在 WASM 中完成，隐私优先。适用于日志分析、跨时区协作、API 调试。

## ✨ 特性

- ⚡ **WASM 原生** —— 所有时间转换和格式化在浏览器中完成，零服务器往返
- 🔒 **隐私优先** —— 时间戳和时区查询绝不离开浏览器
- 🕐 **多格式输出** —— ISO 8601、RFC 2822、RFC 3339、自定义 strftime 格式
- 🌍 **多时区支持** —— 内置 16 个常用时区，支持自定义偏移量
- 🔢 **智能识别** —— 自动识别秒级和毫秒级 Unix 时间戳
- ⏱️ **时长计算** —— 两个时间戳之间的精确时间差
- 📋 **一键复制** —— 每个输出都有独立的复制按钮

## 🚀 快速开始

```bash
# 克隆
git clone https://github.com/Tinkora/timestamp_zone.git
cd timestamp_zone

# 构建 Web WASM
wasm-pack build --target web crates/timestamp_zone_web

# 启动
cp crates/timestamp_zone_web/pkg/* crates/timestamp_zone_web/static/pkg/
cd crates/timestamp_zone_web/static && python3 -m http.server 8080
```

浏览器打开 `http://localhost:8080`

## 📂 项目结构

| 组件 | 说明 | 状态 |
|------|------|------|
| `timestamp_zone_core` | 转换逻辑、时区数据、错误类型 | ✅ |
| `timestamp_zone_web` | WASM 桥接 + HTML 转换器 UI | ✅ |
| `skills/` | Agent Skill 定义 (MCP tools) | ✅ |

## 🔧 开发

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p timestamp_zone_web --target wasm32-unknown-unknown
```

## 📄 文档

- [产品规格](docs/product_spec.zh-CN.md)

## 🤝 社区

- [贡献指南](./CONTRIBUTING.md)
- [行为准则](./CODE_OF_CONDUCT.md)
- [安全策略](./SECURITY.md)
- [更新日志](./CHANGELOG.md)

## Support

If timestamp_zone saves you time, support Tinkora on [Ko-fi](https://ko-fi.com/tinkora).
Support is optional and never affects access or issue priority.

See [SUPPORT.md](./SUPPORT.md) for questions, bug reports, and security reports.

## 📜 License

MIT © [Tinkora](https://github.com/Tinkora)
