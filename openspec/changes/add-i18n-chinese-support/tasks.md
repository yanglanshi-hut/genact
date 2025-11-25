# Implementation Tasks

## 1. 基础设施搭建

- [x] 1.1 在 `Cargo.toml` 中添加 `rust-i18n` 依赖
- [x] 1.2 创建翻译文件目录结构 `locales/en/` 和 `locales/zh-CN/`
- [x] 1.3 创建 `src/i18n.rs` 模块文件
- [x] 1.4 在 `src/lib.rs` 中添加 `pub mod i18n;` 声明
- [x] 1.5 配置 `rust-i18n` 初始化宏(在 `src/i18n.rs` 中)

## 2. 语言检测与切换

- [x] 2.1 在 `src/args.rs` 的 `AppConfig` 中添加 `lang: Option<String>` 字段
- [x] 2.2 在 `src/args.rs` 中添加 `--lang` / `-L` 命令行参数定义
- [x] 2.3 实现 `src/i18n.rs::detect_language()` 函数(检测顺序:CLI参数 → GENACT_LANG → LANG → 默认en)
- [x] 2.4 实现 `src/i18n.rs::init()` 函数,在程序启动时调用
- [x] 2.5 在 `src/main.rs` 和 WASM 入口中调用 `i18n::init()`

## 3. CLI 帮助信息翻译

- [x] 3.1 创建 `locales/en/cli.yml` - 英文翻译(作为基准)
- [x] 3.2 创建 `locales/zh-CN/cli.yml` - 中文翻译
- [x] 3.3 翻译应用描述(about)
- [x] 3.4 翻译所有命令行参数的 `help` 文本
  - [x] `--list-modules`
  - [x] `--modules`
  - [x] `--speed-factor`
  - [x] `--instant-print-lines`
  - [x] `--exit-after-time`
  - [x] `--exit-after-modules`
  - [x] `--print-completions`
  - [x] `--print-manpage`
  - [x] `--lang` (新增)
- [x] 3.5 修改 `src/args.rs` 中所有 `#[clap()]` 属性使用 `t!()` 宏

## 4. 错误和提示消息翻译

- [x] 4.1 创建 `locales/en/messages.yml` 和 `locales/zh-CN/messages.yml`
- [x] 4.2 翻译 `src/args.rs` 中的错误消息
  - [x] "Speed factor must be larger than 0.01"
  - [x] "Must be larger than 0"
- [x] 4.3 翻译 `src/main.rs` 中的提示消息
  - [x] "Available modules:"
- [x] 4.4 翻译 `src/lib.rs` 中的退出消息
  - [x] "Saving work to disk..."
- [x] 4.5 修改相关代码使用 `t!()` 宏

## 5. 模块输出翻译(可选 - Phase 2)

- [x] 5.1 创建 `locales/en/modules.yml` 和 `locales/zh-CN/modules.yml`
- [x] 5.2 提取并翻译通用状态词
  - [x] "Downloading"
  - [x] "Compiling"
  - [x] "Finished"
  - [x] "Building"
  - [x] "Removing"
  - [x] 其他模块特定术语
- [x] 5.3 修改 `src/modules/cargo.rs` 使用翻译宏
- [x] 5.4 修改 `src/modules/composer.rs` 使用翻译宏
- [x] 5.5 修改 `src/modules/docker_build.rs` 使用翻译宏
- [x] 5.6 修改 `src/modules/docker_image_rm.rs` 使用翻译宏
- [x] 5.7 修改其他需要翻译的模块(按需评估)

## 6. WASM 支持(可选 - Phase 3)

- [x] 6.1 验证 `rust-i18n` 在 WASM 环境下编译通过
- [x] 6.2 实现 WASM 环境的语言检测(浏览器语言 API)
- [x] 6.3 添加 URL 参数解析支持 `?lang=zh-CN`
- [x] 6.4 在 `index.html` 中添加语言切换说明或 UI

## 7. 测试与验证

- [ ] 7.1 编译原生二进制: `cargo build --release`
- [ ] 7.2 编译 WASM: `cargo build --target wasm32-unknown-unknown`
- [ ] 7.3 测试默认英文: `./genact --help`
- [ ] 7.4 测试中文切换: `./genact --lang zh-CN --help`
- [ ] 7.5 测试环境变量: `GENACT_LANG=zh-CN ./genact --help`
- [ ] 7.6 测试系统语言检测(在中文系统上)
- [ ] 7.7 测试所有模块运行正常: `./genact --lang zh-CN -m cargo`
- [ ] 7.8 验证二进制大小增加 < 50KB
- [ ] 7.9 检查无翻译缺失警告(编译时)

## 8. 文档更新

- [ ] 8.1 更新 `README.md` 添加语言切换说明
  - [ ] 在 "Usage" 章节添加 `--lang` 参数说明
  - [ ] 添加示例: `genact --lang zh-CN`
  - [ ] 说明环境变量方式
- [ ] 8.2 更新 `CONTRIBUTING.md` 添加翻译贡献指南
  - [ ] 说明翻译文件位置
  - [ ] 说明如何添加新语言
  - [ ] 说明翻译 key 命名规范
- [ ] 8.3 创建 `README.zh-CN.md`(可选,完整的中文 README)
- [ ] 8.4 更新 `CHANGELOG.md` 记录此功能

## 9. 翻译质量保证

- [ ] 9.1 由至少 1 名中文母语者审校所有翻译
- [ ] 9.2 确保专业术语翻译一致性
- [ ] 9.3 确保语气和风格符合项目调性(轻松、幽默)
- [ ] 9.4 检查是否有遗漏的可翻译字符串
- [ ] 9.5 验证翻译覆盖率 > 90%

## 10. CI/CD 集成

- [ ] 10.1 确保 GitHub Actions 编译流程包含 WASM 目标
- [ ] 10.2 添加翻译完整性检查(如果 rust-i18n 支持)
- [ ] 10.3 验证所有平台(Linux, macOS, Windows, FreeBSD)编译通过
- [ ] 10.4 更新 Docker 镜像构建配置(确保包含翻译)

## 11. 发布准备

- [ ] 11.1 更新版本号(遵循语义化版本)
- [ ] 11.2 准备发布说明(中英文)
- [ ] 11.3 生成新版本的 man page 和 shell completions
- [ ] 11.4 测试预发布二进制文件
- [ ] 11.5 准备社区公告(Reddit, Hacker News 等)

## 备注

### 依赖关系
- 任务 2 依赖于任务 1
- 任务 3, 4, 5 可以并行进行,但都依赖于任务 2
- 任务 6 依赖于任务 1, 2 完成
- 任务 7 依赖于所有实现任务完成
- 任务 8, 9 可以与实现并行进行
- 任务 10, 11 在所有任务完成后进行

### 优先级
- **P0 (必须)**: 任务 1-4, 7-9
- **P1 (推荐)**: 任务 5 (模块输出翻译)
- **P2 (可选)**: 任务 6 (WASM 支持)
- **P3 (后续)**: 任务 8.3 (完整中文 README)

### 估算工作量
- Phase 1 (CLI 基础): ~4-6 小时
- Phase 2 (模块输出): ~2-4 小时
- Phase 3 (WASM): ~2-3 小时
- 测试与文档: ~2-3 小时
- **总计**: ~10-16 小时
