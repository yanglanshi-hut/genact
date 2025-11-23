# Project Context

## Purpose

genact 是一个娱乐性质的"无意义活动生成器"命令行工具。它能够模拟各种看起来很忙碌的终端活动(如代码编译、包管理、系统部署等),但实际上不执行任何真实操作。

**核心目标:**
- 提供逼真的终端活动模拟,用于演示、娱乐或"装忙"场景
- 支持多种常见开发工具和系统操作的模拟场景
- 提供原生和 Web 两种使用方式,方便不同场景下使用
- 保持轻量级和高性能

## Tech Stack

### 核心技术
- **语言**: Rust (Edition 2024)
- **异步运行时**: async-std
- **命令行解析**: clap (仅原生)
- **WebAssembly**: wasm-bindgen, web-sys (Web 版本)

### 主要依赖
- **随机数生成**: rand, rand_distr, fake
- **时间处理**: chrono, instant, humantime
- **终端输出**: yansi (颜色), terminal_size
- **数据处理**: regex, sha2, lazy_static
- **其他**: anyhow (错误处理), getrandom (跨平台随机数)

### 构建工具
- Cargo (Rust 包管理器)
- Docker (容器化部署)
- GitHub Actions (CI/CD)

## Project Conventions

### Code Style

- **Rust 标准**: 遵循 Rust 2024 Edition 规范
- **格式化**: 使用 `.editorconfig` 统一编辑器配置
- **命名约定**:
  - 模块文件使用下划线命名: `docker_build.rs`, `kernel_compile.rs`
  - 结构体使用 PascalCase
  - 函数和变量使用 snake_case
- **异步标记**: 所有模块的 `run` 方法必须是异步的 (`async fn`)

### Architecture Patterns

**模块化架构**:
```
src/
├── main.rs          # 入口点(仅原生)
├── lib.rs           # 核心库(原生 + WASM 共享)
├── args.rs          # 配置和参数解析
├── data.rs          # 数据加载
├── generators.rs    # 辅助生成器
├── io.rs           # I/O 抽象层
└── modules/        # 模拟场景模块
    ├── mod.rs      # 模块注册和接口定义
    ├── ansible.rs
    ├── cargo.rs
    └── ...
```

**核心设计**:
- 每个模拟场景作为独立模块实现 `Module` trait
- 使用 `lazy_static` 管理全局状态(速度因子、退出标志等)
- 通过 `cfg` 属性区分原生和 WASM 代码路径
- 静态数据文件存放在 `data/` 目录

### Testing Strategy

- 手动测试为主(由于项目性质为模拟输出)
- 确保所有模块在原生和 Web 环境下都能正常运行
- 测试不同速度因子下的行为
- 验证各平台的编译和运行

### Git Workflow

- **主分支**: `master` 用于稳定发布
- **提交规范**: 清晰描述变更内容
- **PR 模板**: 使用 `PULL_REQUEST_TEMPLATE.md`
- **版本管理**: 使用 `cargo-release` 管理版本发布
- **CI/CD**: GitHub Actions 自动构建和发布

## Domain Context

### 模拟场景类别

1. **包管理器**: cargo, composer
2. **容器化**: docker_build, docker_image_rm
3. **配置管理**: ansible, terraform
4. **系统操作**: bootlog, mkinitcpio, kernel_compile
5. **安全工具**: rkhunter, bruteforce
6. **开发工具**: cc (C 编译器), julia
7. **其他**: simcity, cryptomining, botnet, weblog, download, memdump

### 数据文件说明

`data/` 目录包含各模块使用的静态数据:
- `ansible_tasks.txt`: Ansible 任务列表
- `docker_packages.txt`: Docker 包名称
- `terraform_*_resources.txt`: 云平台资源类型
- `bootlog.txt`: 启动日志模板
- 等等

### 可用模块列表

截至当前版本,genact 支持以下模拟场景:
- ansible - Ansible playbook 执行
- bootlog - 系统启动日志
- botnet - 僵尸网络活动模拟
- bruteforce - 暴力破解模拟
- cargo - Rust 包编译
- cc - C 编译器
- composer - PHP 包管理
- cryptomining - 加密货币挖矿
- docker_build - Docker 镜像构建
- docker_image_rm - Docker 镜像删除
- download - 文件下载
- julia - Julia 科学计算
- kernel_compile - Linux 内核编译
- memdump - 内存转储
- mkinitcpio - Arch Linux initramfs 生成
- rkhunter - Rootkit 扫描
- simcity - SimCity 风格城市模拟
- terraform - Terraform 基础设施部署
- weblog - Web 服务器日志

## Important Constraints

### 技术约束
- **编译目标**: 必须同时支持原生二进制和 `wasm32-unknown-unknown`
- **依赖限制**: WASM 目标不能使用标准库的某些功能(如文件系统、线程等)
- **二进制大小**: Release 构建使用 LTO 和大小优化(`opt-level = 'z'`)
- **平台兼容性**: 代码必须在 FreeBSD, Linux, macOS, Windows 上编译和运行

### 性能要求
- 模拟活动必须流畅,避免卡顿
- 支持速度因子调整(加快/减慢模拟速度)
- 内存占用保持低水平

### 用户体验约束
- 所有模拟输出必须看起来真实可信
- 支持 Ctrl+C 优雅退出
- 提供清晰的命令行选项和帮助信息

## External Dependencies

### 运行时依赖
- **原生**: 无额外运行时依赖(静态链接)
- **Web**: 需要支持 WebAssembly 的现代浏览器

### 数据文件
- 所有模拟数据嵌入到二进制文件中(`data/` 目录)
- 不依赖外部 API 或网络服务

### 发布平台
- **Crates.io**: Rust 包仓库
- **GitHub Releases**: 预编译二进制文件
- **Docker Hub**: Docker 镜像
- **包管理器**: Homebrew, MacPorts, Scoop, FreeBSD pkg

## Building and Running

### 本地开发
```bash
# 克隆仓库
git clone https://github.com/svenstaro/genact.git
cd genact

# 运行(开发模式)
cargo run

# 运行(发布模式)
cargo run --release

# 构建
cargo build --release

# 查看所有选项
cargo run -- -h
```

### 运行特定模块
```bash
# 列出所有可用模块
./genact --list-modules

# 运行特定模块
./genact -m cargo -m docker_build

# 调整速度因子
./genact --speed-factor 2

# 立即打印前 N 行(跳过动画)
./genact --instant-print-lines 10

# 运行指定时间后退出
./genact --exit-after-time 5min

# 运行指定次数后退出
./genact --exit-after-modules 3
```

### Web 版本
- 访问: https://svenstaro.github.io/genact
- URL 参数:
  - `?module=cc&module=memdump` - 指定模块
  - `?speed-factor=5` - 设置速度因子

### Docker 使用
```bash
# 运行默认配置
docker run -it --rm svenstaro/genact

# 查看帮助
docker run -it --rm svenstaro/genact -h

# 运行特定模块
docker run -it --rm svenstaro/genact -m cargo
```

### 发布流程
```bash
# 更新 CHANGELOG.md
# 执行发布命令
cargo release <version>
cargo release --execute <version>
# GitHub Actions 将自动部署
```

## Development Guidelines

### 添加新模块

1. 在 `src/modules/` 创建新的 `.rs` 文件
2. 实现 `Module` trait:
```rust
use async_trait::async_trait;
use crate::args::AppConfig;
use crate::modules::Module;

pub struct MyModule;

#[async_trait(?Send)]
impl Module for MyModule {
    fn name(&self) -> &'static str {
        "mymodule"
    }

    async fn run(&self, appconfig: &AppConfig) {
        // 实现模拟逻辑
    }
}
```
3. 在 `src/modules/mod.rs` 中注册模块
4. 如需数据文件,添加到 `data/` 目录

### 跨平台注意事项

- 使用 `#[cfg(not(target_arch = "wasm32"))]` 标记原生代码
- 使用 `#[cfg(target_arch = "wasm32")]` 标记 WASM 代码
- 避免在 WASM 中使用文件系统、线程等功能
- 使用 `instant` crate 而非 `std::time::Instant`

### 代码规范

- 运行 `cargo fmt` 格式化代码
- 运行 `cargo clippy` 检查潜在问题
- 确保在所有目标平台上编译通过:
  ```bash
  cargo build --release
  cargo build --target wasm32-unknown-unknown
  ```

## Project Structure

### 核心文件
- `Cargo.toml` - 项目配置和依赖
- `src/main.rs` - 原生入口点
- `src/lib.rs` - 共享库代码
- `index.html` - Web 版本入口

### 配置文件
- `.editorconfig` - 编辑器配置
- `Trunk.toml` - Web 构建配置
- `release.toml` - 发布配置
- `Containerfile` - Docker 镜像配置

### 文档和元数据
- `README.md` - 项目说明
- `CHANGELOG.md` - 版本历史
- `CONTRIBUTING.md` - 贡献指南
- `LICENSE` - MIT 许可证
- `PULL_REQUEST_TEMPLATE.md` - PR 模板

### 资源文件
- `data/` - 模拟数据文件
- `gifs/` - 演示动图
- `static/` - Web 静态资源
