# Simple Block Chain

一个极简的区块链原理实现，专为学习和理解区块链核心概念而设计。

## 项目简介

本项目用 Rust 实现了区块链的基础功能，代码简洁、注释清晰，帮助开发者快速理解：
- 区块结构与哈希链接
- 工作量证明（PoW）机制
- 交易的生成与验证
- 确定性数据生成（地点信息示例）

## 核心特性

| 特性 | 说明 |
|------|------|
| 区块结构 | 包含 Header（哈希、时间戳、前一区块哈希）和 Body（交易数据） |
| 链式存储 | 每个区块通过 `pre_hash` 链接，形成不可篡改的链条 |
| 序列化验证 | 使用 Serde+BCS 实现数据序列化与哈希验证 |

## 快速开始

### 运行环境
- Rust 1.70+
- Cargo

### 克隆并运行

```bash
git clone https://github.com/yourname/simple-block-chain.git
cd simple-block-chain
cargo run
