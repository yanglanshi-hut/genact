# Technical Design: 国际化支持

## Context

genact 是一个轻量级的命令行工具和 WebAssembly 应用,目前仅支持英文。为了扩大用户群体,特别是中文开发者社区,我们需要引入国际化(i18n)支持。

### 背景
- **用户群体**: 全球开发者,包括大量中文用户
- **当前限制**: 所有用户界面文本硬编码为英文
- **技术栈**: Rust + async-std,支持原生和 WASM 双编译目标
- **部署方式**: 静态二进制(无外部依赖),Web(纯静态 WASM)

### 约束条件
1. **零运行时依赖**: 不能依赖外部翻译文件或服务
2. **WASM 兼容性**: 必须在浏览器环境中工作
3. **二进制大小**: 保持轻量级,增加 < 50KB
4. **向后兼容**: 默认行为保持不变(英文)
5. **多平台**: Linux, macOS, Windows, FreeBSD, WASM

### 利益相关者
- **最终用户**: 中文开发者获得本地化体验
- **维护者**: 需要维护翻译文件
- **贡献者**: 可以贡献新语言翻译

## Goals / Non-Goals

### Goals
1. ✅ 支持简体中文界面(CLI 帮助、错误消息)
2. ✅ 提供灵活的语言切换机制(CLI 参数、环境变量、自动检测)
3. ✅ 保持向后兼容(默认英文,不影响现有用户)
4. ✅ 最小化性能和大小影响
5. ✅ 为未来支持更多语言奠定基础

### Non-Goals
1. ❌ 完全翻译所有模块输出内容(Phase 1 仅 CLI)
2. ❌ 支持用户自定义翻译文件(静态编译)
3. ❌ 运行时热重载翻译(非需求)
4. ❌ 复杂语法(复数、性别等,暂不需要)
5. ❌ 本提案不包含其他语言(如日文、韩文)

## Decisions

### 决策 1: 选择 rust-i18n 作为 i18n 库

**理由**:
- ✅ **编译时嵌入**: 翻译在编译时嵌入二进制,零运行时依赖
- ✅ **WASM 友好**: 无需文件系统,完美支持 WASM
- ✅ **性能最优**: 零运行时开销,翻译查询是直接内存访问
- ✅ **轻量级**: 仅增加 ~20KB 二进制大小
- ✅ **易用性**: 宏驱动,API 简洁 (`t!("key")`)
- ✅ **YAML 支持**: 翻译文件易读易写

**替代方案**:
- fluent-rs: 功能强大但过于复杂,增加 50-100KB,运行时解析
- 手动实现: 维护成本高,缺少工具链支持

**技术细节**:
```rust
// Cargo.toml
[dependencies]
rust-i18n = "3"

// src/i18n.rs
rust_i18n::i18n!("locales");

// 使用
t!("cli.description")  // 返回当前语言的翻译
```

### 决策 2: 采用分层翻译文件结构

**结构**:
```
locales/
├── en/
│   ├── cli.yml       # CLI 参数和帮助
│   ├── messages.yml  # 错误消息和提示
│   └── modules.yml   # 模块输出(Phase 2)
└── zh-CN/
    ├── cli.yml
    ├── messages.yml
    └── modules.yml
```

**理由**:
- ✅ **清晰分类**: 按功能区域组织,易于查找和维护
- ✅ **并行开发**: 不同团队可独立翻译不同文件
- ✅ **渐进迭代**: Phase 1 仅需 `cli.yml` 和 `messages.yml`
- ✅ **扩展性**: 添加新语言只需复制目录

**替代方案**:
- 单一大文件: 难以维护,合并冲突多
- 按模块拆分: 过度细化,增加查找成本

**示例**:
```yaml
# locales/zh-CN/cli.yml
cli:
  description: 一个无意义活动生成器
  list_modules: 列出可用的模块
  modules: 仅运行这些模块
  speed_factor: 全局速度因子
```

### 决策 3: 多级语言检测策略

**优先级(从高到低)**:
1. CLI 参数 `--lang zh-CN`
2. 环境变量 `GENACT_LANG=zh-CN`
3. 系统语言环境 `LANG=zh_CN.UTF-8`
4. 默认 `en`

**理由**:
- ✅ **用户控制**: 明确参数优先级最高
- ✅ **灵活性**: 多种指定方式满足不同场景
- ✅ **自动化**: 系统语言检测提升开箱体验
- ✅ **安全回退**: 默认英文确保兼容性

**实现**:
```rust
pub fn detect_language(args: &AppConfig) -> &'static str {
    // 1. CLI 参数
    if let Some(ref lang) = args.lang {
        return Box::leak(lang.clone().into_boxed_str());
    }
    
    // 2. GENACT_LANG 环境变量
    if let Ok(lang) = std::env::var("GENACT_LANG") {
        return Box::leak(lang.into_boxed_str());
    }
    
    // 3. 系统 LANG 环境变量
    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(lang) = std::env::var("LANG") {
        if lang.starts_with("zh") {
            return "zh-CN";
        }
    }
    
    // 4. WASM: 检测浏览器语言
    #[cfg(target_arch = "wasm32")]
    {
        // navigator.language
        // 实现省略
    }
    
    // 5. 默认英文
    "en"
}
```

### 决策 4: 渐进式翻译范围

**Phase 1: CLI 基础设施**(必须)
- CLI 帮助信息
- 错误消息
- 系统提示

**Phase 2: 模块输出**(推荐)
- 通用状态词("Downloading", "Compiling")
- 保持技术术语英文(包名、路径等)

**Phase 3: Web 支持**(可选)
- 浏览器语言检测
- URL 参数 `?lang=zh-CN`

**理由**:
- ✅ **风险管理**: 分阶段降低复杂度
- ✅ **快速交付**: Phase 1 即可提供价值
- ✅ **真实度**: 模块输出翻译需谨慎,避免破坏"逼真感"
- ✅ **灵活性**: 可根据反馈调整 Phase 2/3

### 决策 5: 翻译 Key 命名规范

**规范**:
```yaml
# 格式: <category>.<subcategory>.<item>
cli.description              # 应用描述
cli.help.list_modules        # --list-modules 帮助
cli.help.speed_factor        # --speed-factor 帮助
messages.error.invalid_speed # 错误:无效速度
messages.info.saving_work    # 提示:保存工作
modules.common.downloading   # 通用:正在下载
modules.cargo.compiling      # Cargo:编译中
```

**理由**:
- ✅ **一致性**: 统一的命名模式易于理解
- ✅ **可搜索**: 分层命名支持前缀搜索
- ✅ **避免冲突**: 命名空间隔离
- ✅ **自文档化**: Key 名称即语义

## Risks / Trade-offs

### 风险 1: 翻译质量不一致
- **描述**: 机器翻译或不准确的翻译影响用户体验
- **影响**: 中等 - 可能导致困惑或误解
- **缓解**:
  - ✅ 初始翻译由母语者审校
  - ✅ 建立翻译指南(术语表、风格)
  - ✅ 接受社区 PR 改进翻译
  - ✅ 提供翻译反馈渠道
- **责任人**: 维护者 + 社区

### 风险 2: 维护成本增加
- **描述**: 每次添加新消息需同步所有语言
- **影响**: 低-中等 - 增加开发工作量
- **缓解**:
  - ✅ rust-i18n 编译时警告缺失翻译
  - ✅ 回退到英文(不会阻塞功能)
  - ✅ 在 CONTRIBUTING.md 中说明翻译流程
  - ✅ CI 检查翻译完整性(未来可添加)
- **责任人**: 核心贡献者

### 风险 3: 二进制大小膨胀
- **描述**: 嵌入翻译增加可执行文件大小
- **影响**: 低 - 对用户体验影响小
- **当前预估**: +20KB (rust-i18n) + ~10-30KB (翻译内容)
- **缓解**:
  - ✅ 仅翻译必要内容(Phase 1)
  - ✅ 使用简洁翻译,避免冗长
  - ✅ 监控发布大小,设定阈值(< 50KB)
- **责任人**: 发布管理者

### 风险 4: WASM 兼容性问题
- **描述**: i18n 库可能不支持 WASM
- **影响**: 高 - 阻塞 Web 版本
- **缓解**:
  - ✅ rust-i18n 已验证支持 WASM
  - ✅ CI 添加 WASM 编译测试
  - ✅ 必要时为 WASM 提供简化实现
- **状态**: 已验证,rust-i18n 支持 WASM

### Trade-off 分析

| 方面 | 增益 | 损失 | 结论 |
|------|------|------|------|
| **用户体验** | 中文用户体验显著提升 | 英文用户无影响 | ✅ 净收益 |
| **开发成本** | 一次性集成成本 | 持续维护翻译 | ⚖️ 可接受 |
| **二进制大小** | 无 | +30-50KB | ✅ 可接受 |
| **性能** | 无 | 几乎为零(编译时) | ✅ 无影响 |
| **复杂度** | 更灵活的配置 | 代码轻微复杂 | ✅ 净收益 |

## Migration Plan

### 对现有用户
**无需迁移** - 向后兼容:
- 默认行为保持英文
- 无需修改脚本或配置
- 所有现有命令继续工作

### 对中文用户
**新功能可选使用**:
```bash
# 方式 1: CLI 参数
genact --lang zh-CN

# 方式 2: 环境变量(持久化)
export GENACT_LANG=zh-CN
genact

# 方式 3: 自动检测(中文系统)
genact  # 自动显示中文
```

### 对贡献者
**开发流程调整**:
1. 添加新用户可见字符串时:
   ```rust
   // 旧方式
   println!("Downloading package");
   
   // 新方式
   println!("{}", t!("modules.common.downloading"));
   ```

2. 更新翻译文件:
   ```yaml
   # locales/en/modules.yml
   modules:
     common:
       downloading: Downloading
   
   # locales/zh-CN/modules.yml
   modules:
     common:
       downloading: 正在下载
   ```

3. 编译时检查:
   ```bash
   cargo build  # 会警告缺失翻译
   ```

### 对包维护者
**发布流程**:
- 无变更,GitHub Actions 自动处理
- 翻译已嵌入二进制
- 更新 release notes 说明新功能

## Implementation Details

### 代码集成点

#### 1. Cargo.toml
```toml
[dependencies]
rust-i18n = "3"

[build-dependencies]
rust-i18n = "3"
```

#### 2. src/i18n.rs (新文件)
```rust
use rust_i18n::t;

// 初始化宏,编译时加载 locales/ 目录
rust_i18n::i18n!("locales", fallback = "en");

use crate::args::AppConfig;

/// 检测并设置语言
pub fn init(args: &AppConfig) {
    let lang = detect_language(args);
    rust_i18n::set_locale(lang);
}

/// 语言检测逻辑
fn detect_language(args: &AppConfig) -> &'static str {
    // 实现见"决策 3"
    // ...
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fallback() {
        rust_i18n::set_locale("en");
        assert_eq!(t!("cli.description"), "A nonsense activity generator");
    }
}
```

#### 3. src/main.rs
```rust
use genact::i18n;

#[async_std::main]
async fn main() -> Result<()> {
    let appconfig = parse_args();
    
    // 初始化 i18n
    i18n::init(&appconfig);
    
    // ... 其余代码不变
}
```

#### 4. src/args.rs
```rust
use rust_i18n::t;

#[derive(Parser)]
#[clap(
    name = "genact",
    author,
    about = t!("cli.description"),  // 使用翻译
    version
)]
pub struct AppConfig {
    /// 语言设置
    #[clap(
        short = 'L',
        long = "lang",
        help = t!("cli.help.lang"),
        env = "GENACT_LANG"
    )]
    pub lang: Option<String>,
    
    // ... 其他字段使用 t!() 宏
}
```

### 翻译文件示例

#### locales/en/cli.yml
```yaml
cli:
  description: A nonsense activity generator
  help:
    list_modules: List available modules
    modules: Run only these modules
    speed_factor: Global speed factor
    instant_print_lines: Instantly print this many lines
    exit_after_time: Exit after running for this long
    exit_after_modules: Exit after running this many modules
    print_completions: Generate completion file for a shell
    print_manpage: Generate man page
    lang: Set interface language
```

#### locales/zh-CN/cli.yml
```yaml
cli:
  description: 一个无意义活动生成器
  help:
    list_modules: 列出可用的模块
    modules: 仅运行这些模块
    speed_factor: 全局速度因子
    instant_print_lines: 立即打印指定行数
    exit_after_time: 运行指定时间后退出(格式示例:2h10min)
    exit_after_modules: 运行指定次数后退出
    print_completions: 为指定 shell 生成补全文件
    print_manpage: 生成 man 手册页
    lang: 设置界面语言
```

### WASM 特殊处理

```rust
// src/i18n.rs
#[cfg(target_arch = "wasm32")]
fn detect_browser_language() -> Option<&'static str> {
    use wasm_bindgen::JsCast;
    use web_sys::window;
    
    let window = window()?;
    let navigator = window.navigator();
    let lang = navigator.language()?;
    
    if lang.starts_with("zh") {
        Some("zh-CN")
    } else {
        Some("en")
    }
}
```

## Testing Strategy

### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_translation_keys_exist() {
        rust_i18n::set_locale("en");
        assert!(!t!("cli.description").is_empty());
        
        rust_i18n::set_locale("zh-CN");
        assert!(!t!("cli.description").is_empty());
    }
    
    #[test]
    fn test_language_detection() {
        // 测试各种检测场景
    }
}
```

### 集成测试
```bash
# 测试默认英文
./genact --help | grep "A nonsense activity generator"

# 测试中文
./genact --lang zh-CN --help | grep "一个无意义活动生成器"

# 测试环境变量
GENACT_LANG=zh-CN ./genact --help | grep "一个无意义活动生成器"
```

### 编译测试
```bash
# 原生
cargo build --release
cargo test

# WASM
cargo build --target wasm32-unknown-unknown
```

## Monitoring and Metrics

### 发布前检查
- [ ] 编译通过(所有平台)
- [ ] 二进制大小 < 基准 + 50KB
- [ ] 翻译覆盖率 > 90%
- [ ] 手动测试语言切换

### 发布后监控
- 用户反馈(GitHub Issues)
- 下载量趋势(中文用户增长)
- 翻译贡献 PR 数量

## Open Questions

### Q1: 是否需要翻译模块输出?
**状态**: ❓ 待定  
**影响**: Phase 2 范围  
**讨论**:
- 赞成: 提供完整本地化体验
- 反对: 可能降低"逼真度"(技术输出通常英文)
- **建议**: Phase 1 实现后收集用户反馈

### Q2: Web 版本是否同步支持?
**状态**: ❓ 待定  
**影响**: Phase 3 范围  
**讨论**:
- 技术上可行(rust-i18n 支持 WASM)
- 增加测试复杂度
- **建议**: Phase 3 独立评估

### Q3: 是否支持繁体中文(zh-TW)?
**状态**: ❓ 待定  
**影响**: 工作量  
**讨论**:
- 简繁差异较小,可通过工具转换
- 增加维护负担
- **建议**: 先支持 zh-CN,后续接受社区贡献

### Q4: 是否需要翻译校验 CI?
**状态**: ❓ 待定  
**影响**: CI 复杂度  
**讨论**:
- rust-i18n 编译时已有基本检查
- 可添加更严格的覆盖率检查
- **建议**: Phase 1 后根据需要添加

## References

### 技术文档
- [rust-i18n GitHub](https://github.com/longbridgeapp/rust-i18n)
- [rust-i18n 文档](https://docs.rs/rust-i18n)
- [WASM 国际化最佳实践](https://rustwasm.github.io/book/)

### 类似项目
- [ripgrep](https://github.com/BurntSushi/ripgrep) - 无 i18n(仅英文)
- [exa](https://github.com/ogham/exa) - 无 i18n
- [starship](https://github.com/starship/starship) - 有 i18n(使用 fluent)

### 参考资料
- [Rust i18n 生态对比](https://lib.rs/keywords/i18n)
- [Mozilla Fluent 标准](https://projectfluent.org/)
- [Unicode CLDR](https://cldr.unicode.org/)
