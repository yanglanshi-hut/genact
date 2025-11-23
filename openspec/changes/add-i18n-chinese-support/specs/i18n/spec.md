## ADDED Requirements

### Requirement: 语言检测和初始化

系统 SHALL 在启动时检测并设置用户界面语言,支持多种语言指定方式。

#### Scenario: 使用 CLI 参数指定语言

- **WHEN** 用户运行 `genact --lang zh-CN`
- **THEN** 所有用户界面文本显示为简体中文
- **AND** 语言设置优先于环境变量和系统语言

#### Scenario: 使用环境变量指定语言

- **WHEN** 用户设置 `GENACT_LANG=zh-CN` 环境变量并运行 `genact`
- **THEN** 所有用户界面文本显示为简体中文
- **AND** 环境变量优先于系统语言检测

#### Scenario: 自动检测系统语言

- **WHEN** 用户未指定语言参数且未设置 GENACT_LANG 环境变量
- **AND** 系统 LANG 环境变量为 `zh_CN.UTF-8` 或类似中文区域设置
- **THEN** 系统自动选择简体中文界面

#### Scenario: 默认回退到英文

- **WHEN** 用户未指定语言且系统语言无法识别
- **THEN** 系统使用英文作为默认界面语言

#### Scenario: 无效语言代码处理

- **WHEN** 用户指定不支持的语言代码(如 `--lang fr`)
- **THEN** 系统回退到英文并继续运行
- **AND** 不显示错误或警告(静默回退)

### Requirement: CLI 帮助信息国际化

系统 SHALL 支持命令行帮助信息的多语言显示。

#### Scenario: 中文帮助信息

- **WHEN** 用户运行 `genact --lang zh-CN --help`
- **THEN** 应用描述显示为"一个无意义活动生成器"
- **AND** 所有命令行参数说明显示为中文
- **AND** 参数名称保持英文(如 `--speed-factor`)

#### Scenario: 英文帮助信息

- **WHEN** 用户运行 `genact --help` 或 `genact --lang en --help`
- **THEN** 所有帮助文本显示为英文
- **AND** 与未添加国际化功能前的输出一致

#### Scenario: 参数翻译完整性

- **WHEN** 系统显示帮助信息
- **THEN** 以下所有参数说明 MUST 被翻译:
  - `--list-modules` / `-l`
  - `--modules` / `-m`
  - `--speed-factor` / `-s`
  - `--instant-print-lines` / `-i`
  - `--exit-after-time`
  - `--exit-after-modules`
  - `--print-completions`
  - `--print-manpage`
  - `--lang` / `-L` (新增参数)

### Requirement: 错误和提示消息国际化

系统 SHALL 支持运行时错误消息和提示信息的多语言显示。

#### Scenario: 中文错误消息

- **WHEN** 用户使用中文界面并提供无效参数(如 `--speed-factor 0`)
- **THEN** 错误消息显示为中文(如"速度因子必须大于 0.01")

#### Scenario: 中文系统提示

- **WHEN** 用户使用中文界面并运行 `genact --list-modules`
- **THEN** 输出标题显示为"可用模块:"
- **AND** 模块名称列表保持英文(技术标识符)

#### Scenario: 退出消息国际化

- **WHEN** 用户按 Ctrl+C 退出程序
- **AND** 当前使用中文界面
- **THEN** 显示"正在将工作保存到磁盘..."(或类似中文提示)

### Requirement: 翻译资源管理

系统 SHALL 使用结构化的翻译文件管理多语言内容。

#### Scenario: 翻译文件结构

- **WHEN** 系统编译时
- **THEN** 翻译文件 MUST 位于 `locales/` 目录
- **AND** 每种语言有独立子目录(如 `locales/en/`, `locales/zh-CN/`)
- **AND** 翻译文件使用 YAML 格式并按功能分类(如 `cli.yml`, `messages.yml`)

#### Scenario: 翻译键命名规范

- **WHEN** 开发者添加新的可翻译字符串
- **THEN** 翻译键 MUST 遵循格式 `<category>.<subcategory>.<item>`
- **AND** 使用小写字母和下划线(如 `cli.help.speed_factor`)

#### Scenario: 缺失翻译回退

- **WHEN** 某个翻译键在当前语言中不存在
- **THEN** 系统 MUST 回退到英文翻译
- **AND** 程序继续正常运行(不崩溃或抛出错误)

#### Scenario: 编译时翻译嵌入

- **WHEN** 执行 `cargo build --release`
- **THEN** 所有翻译内容 MUST 嵌入到二进制文件中
- **AND** 运行时不依赖外部翻译文件

### Requirement: 语言参数配置

系统 SHALL 提供 `--lang` 命令行参数用于手动指定界面语言。

#### Scenario: 长格式参数

- **WHEN** 用户运行 `genact --lang zh-CN`
- **THEN** 系统使用简体中文界面

#### Scenario: 短格式参数

- **WHEN** 用户运行 `genact -L zh-CN`
- **THEN** 系统使用简体中文界面(与长格式效果相同)

#### Scenario: 参数值格式

- **WHEN** 系统接受 `--lang` 参数值
- **THEN** MUST 支持以下格式:
  - `en` - 英文
  - `zh-CN` - 简体中文
- **AND** 语言代码大小写不敏感(如 `zh-cn` 等效于 `zh-CN`)

#### Scenario: 环境变量支持

- **WHEN** 用户未提供 `--lang` 参数
- **AND** `GENACT_LANG` 环境变量已设置
- **THEN** 系统使用环境变量指定的语言
- **AND** 环境变量值格式与 CLI 参数一致

### Requirement: 向后兼容性

系统 SHALL 确保国际化功能不影响现有用户的使用体验。

#### Scenario: 默认英文界面

- **WHEN** 用户未指定任何语言设置
- **AND** 系统语言不是中文
- **THEN** 系统显示英文界面(与添加 i18n 前一致)

#### Scenario: 现有脚本兼容

- **WHEN** 用户使用现有脚本或自动化工具调用 genact
- **AND** 脚本未指定 `--lang` 参数
- **THEN** 所有命令和输出保持原有行为
- **AND** 不产生额外的警告或提示

#### Scenario: 命令行参数不冲突

- **WHEN** 用户同时使用新旧参数(如 `genact -m cargo --lang zh-CN`)
- **THEN** 所有参数正常工作且互不冲突

### Requirement: WASM 环境支持

系统 SHALL 在 WebAssembly 环境中支持国际化功能。

#### Scenario: WASM 编译成功

- **WHEN** 执行 `cargo build --target wasm32-unknown-unknown`
- **THEN** 编译成功且包含所有翻译资源
- **AND** WASM 二进制大小增加 < 50KB

#### Scenario: 浏览器语言检测 (可选 - Phase 3)

- **WHEN** 用户在浏览器中访问 Web 版本
- **AND** 浏览器语言设置为中文
- **THEN** 系统自动显示中文界面
- **AND** 检测逻辑使用 `navigator.language` API

#### Scenario: URL 参数语言切换 (可选 - Phase 3)

- **WHEN** 用户访问 `https://example.com/genact?lang=zh-CN`
- **THEN** 系统显示中文界面
- **AND** URL 参数优先于浏览器语言检测

### Requirement: 性能和大小约束

系统 SHALL 确保国际化功能对性能和二进制大小的影响最小化。

#### Scenario: 二进制大小增加限制

- **WHEN** 编译包含国际化支持的 Release 版本
- **THEN** 二进制文件大小增加 MUST < 50KB(相比未添加 i18n 前)

#### Scenario: 零运行时开销

- **WHEN** 系统运行时查询翻译
- **THEN** 翻译查询 MUST 是直接内存访问(编译时生成的静态数据)
- **AND** 不涉及文件 I/O 或网络请求
- **AND** 性能影响可忽略不计(< 1%)

#### Scenario: 启动时间影响

- **WHEN** 程序启动并初始化语言设置
- **THEN** 语言检测和设置时间 MUST < 1ms
- **AND** 不显著影响程序启动速度

### Requirement: 翻译质量保证

系统 SHALL 确保翻译内容准确、一致且符合目标语言习惯。

#### Scenario: 专业术语处理

- **WHEN** 翻译包含技术术语(如 "speed factor", "module")
- **THEN** 术语翻译 MUST 保持一致性(全局统一)
- **AND** 使用业界通用译法(如 "速度因子", "模块")

#### Scenario: 命令和参数名称保持英文

- **WHEN** 显示命令行参数或模块名称
- **THEN** 参数名称 MUST 保持英文(如 `--speed-factor` 不翻译为 `--速度因子`)
- **AND** 模块名称保持英文(如 `cargo`, `docker_build`)
- **AND** 仅翻译参数的描述文本

#### Scenario: 母语者审校

- **WHEN** 提交翻译内容
- **THEN** 中文翻译 SHOULD 由至少一名中文母语者审校
- **AND** 确保语句通顺、语气自然

### Requirement: 可扩展性

系统 SHALL 设计为易于添加新语言支持的架构。

#### Scenario: 添加新语言流程

- **WHEN** 贡献者希望添加新语言(如日文)
- **THEN** 只需创建 `locales/ja/` 目录并添加翻译文件
- **AND** 无需修改 Rust 代码(除非添加特殊检测逻辑)
- **AND** 重新编译即可支持新语言

#### Scenario: 翻译文件格式一致性

- **WHEN** 所有语言的翻译文件
- **THEN** MUST 使用相同的 YAML 结构和键命名
- **AND** 每种语言的文件名和路径保持对应(如 `en/cli.yml` 对应 `zh-CN/cli.yml`)

#### Scenario: 翻译覆盖率检查

- **WHEN** 编译时加载翻译文件
- **THEN** 系统 SHOULD 警告缺失的翻译键
- **AND** 提供清晰的错误信息指示哪些键未翻译

### Requirement: 文档和示例

系统 SHALL 提供清晰的文档说明如何使用和贡献国际化功能。

#### Scenario: README 更新

- **WHEN** 用户查看项目 README
- **THEN** MUST 包含 `--lang` 参数的使用说明和示例
- **AND** 说明支持的语言列表

#### Scenario: 贡献指南

- **WHEN** 贡献者查看 CONTRIBUTING.md
- **THEN** MUST 包含翻译贡献指南
- **AND** 说明翻译文件位置、格式和命名规范
- **AND** 提供添加新语言的步骤

#### Scenario: 使用示例

- **WHEN** 文档提供使用示例
- **THEN** MUST 包含以下场景:
  - 使用 CLI 参数切换语言
  - 使用环境变量设置语言
  - 列出支持的语言
