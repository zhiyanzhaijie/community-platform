# Community Trading Platform - Core

社区交易平台核心后端服务，基于 Rust + Axum + DDD 架构。

## 🏗️ 项目结构

```
core/
├── crates/
│   ├── domain/       # 🎯 领域层（纯业务逻辑）
│   ├── app/          # 📋 应用层（用例编排）
│   ├── infra/        # 🔧 基础设施层（数据库、日志）
│   ├── api/          # 🌐 API层（HTTP接口）
│   └── shared/       # 📦 共享库（配置、错误）
├── migrations/       # 数据库迁移文件
├── config/           # 配置文件
└── docs/             # 文档（见 ../docs/）
```

## 🚀 快速开始

### 前置要求

- Rust 1.75+
- PostgreSQL 16+
- mold 链接器（可选，加速编译）

### 安装依赖

```bash
# 安装 sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# 安装 mold（Arch Linux）
sudo pacman -S mold
```

### 设置数据库

```bash
# 设置环境变量
export DATABASE_URL="postgres://postgres:postgres@localhost/community_trading_dev"

# 创建数据库
sqlx database create

# 运行迁移
sqlx migrate run --source migrations
```

### 运行服务

```bash
# 开发模式
cargo run -p api

# 或使用别名
cargo r -p api

# 检查编译（更快）
cargo check --workspace
```

## 📦 Crates 说明

### domain
- **职责**：纯业务逻辑，聚合根、值对象、领域事件
- **依赖**：零业务依赖（仅工具库：serde, uuid, chrono）
- **测试**：`cargo test -p domain`

### app
- **职责**：用例编排、应用服务
- **依赖**：domain
- **测试**：`cargo test -p app`

### infra
- **职责**：数据持久化、日志、外部服务
- **依赖**：domain, app
- **测试**：`cargo test -p infra`

### api
- **职责**：HTTP接口、中间件、DTO
- **依赖**：domain, app, infra, shared
- **运行**：`cargo run -p api`

### shared
- **职责**：配置管理、通用错误类型
- **依赖**：config, dotenvy

## 🛠️ 开发命令

```bash
# 快速检查（不生成二进制）
cargo check -p domain        # 只检查 domain
cargo check --workspace      # 检查所有包

# 运行测试
cargo test -p domain         # 只测试 domain
cargo test --workspace       # 测试所有包

# 构建 release 版本
cargo build --release -p api

# 清理构建缓存
cargo clean
```

## 📊 编译优化

项目使用 mold 链接器加速编译，配置在 `.cargo/config.toml`。

**编译时间对比**：
- 修改 API 层：~5s ⚡️
- 修改 infra 层：~15s
- 修改 app 层：~40s
- 修改 domain 层：~60s

## 🌐 API 接口

服务默认运行在 `http://localhost:3000`

### 健康检查
```bash
curl http://localhost:3000/health
```

### API v1 端点
```
POST   /api/v1/members/register
POST   /api/v1/members/login
GET    /api/v1/members/{id}
...
```

完整 API 文档见 `../docs/api-reference.md`

## 📝 配置

配置文件位于 `config/` 目录：
- `default.toml` - 默认配置
- `development.toml` - 开发环境
- `production.toml` - 生产环境

环境变量优先级更高，可以覆盖配置文件。

## 🧪 测试

```bash
# 运行所有测试
cargo test --workspace

# 运行单个 crate 测试
cargo test -p domain

# 运行特定测试
cargo test test_member_creation

# 查看测试输出
cargo test -- --nocapture
```

## 📚 相关文档

- [架构设计 v2](../docs/architecture-v2.md)
- [错误处理指南](../docs/error-handling-comparison.md)
- [开发路线图](../docs/architecture-v2.md#开发路线图)

## 📄 许可证

待定
