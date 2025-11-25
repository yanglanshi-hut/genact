// src/i18n.rs
//! 国际化(i18n)支持模块
//! 
//! 提供多语言翻译功能,支持简体中文和英文界面。

use rust_i18n::i18n;

// 配置 rust-i18n
// - 翻译文件目录: locales/
// - 可用语言: en(英文), zh-CN(简体中文)
// - 默认语言: en
i18n!("locales", fallback = "en");

/// 检测语言设置
///
/// 按优先级顺序检测语言:
/// 1. CLI 参数 --lang
/// 2. 环境变量 GENACT_LANG
/// 3. 系统 locale
/// 4. 默认 "en"
///
/// # 参数
/// - `cli_lang`: 从命令行参数传入的语言选项
///
/// # 返回值
/// 返回检测到的语言代码字符串（如 "en" 或 "zh-CN"）
#[cfg(not(target_arch = "wasm32"))]
pub fn detect_language(cli_lang: Option<&str>) -> String {
    // 1. 优先使用 CLI 参数
    if let Some(lang) = cli_lang {
        return lang.to_string();
    }
    
    // 2. 检查 GENACT_LANG 环境变量
    if let Ok(lang) = std::env::var("GENACT_LANG") {
        return lang;
    }
    
    // 3. 检测系统 locale
    if let Some(locale) = detect_system_locale() {
        return locale.to_string();
    }
    
    // 4. 默认返回英文
    "en".to_string()
}

/// 检测语言设置 (WASM 版本)
///
/// 按优先级顺序检测语言:
/// 1. URL 参数 ?lang=zh-CN
/// 2. 浏览器语言 (navigator.language)
/// 3. 默认 "en"
///
/// # 参数
/// - `cli_lang`: 从 URL 参数传入的语言选项
///
/// # 返回值
/// 返回检测到的语言代码字符串（如 "en" 或 "zh-CN"）
#[cfg(target_arch = "wasm32")]
pub fn detect_language(cli_lang: Option<&str>) -> String {
    // 1. 优先使用 URL 参数
    if let Some(lang) = cli_lang {
        return lang.to_string();
    }
    
    // 2. 检测浏览器语言
    if let Some(browser_lang) = detect_browser_language() {
        return browser_lang;
    }
    
    // 3. 默认返回英文
    "en".to_string()
}

/// 检测系统语言设置 (原生版本)
///
/// 通过读取系统环境变量 LANG 来判断语言
#[cfg(not(target_arch = "wasm32"))]
fn detect_system_locale() -> Option<&'static str> {
    // 尝试从 LANG 环境变量检测语言
    if let Ok(lang) = std::env::var("LANG") {
        // 检测中文 locale（如 zh_CN.UTF-8）
        if lang.starts_with("zh") {
            return Some("zh-CN");
        }
        // 可以在这里添加更多语言的检测逻辑
    }
    
    None
}

/// 检测浏览器语言设置 (WASM 版本)
///
/// 通过 Web API 获取浏览器的语言偏好
#[cfg(target_arch = "wasm32")]
fn detect_browser_language() -> Option<String> {
    use web_sys::window;
    
    let window = window()?;
    let navigator = window.navigator();
    
    // 获取浏览器语言
    let language = navigator.language()?;
    
    // 标准化语言代码
    // 例如: "zh-CN" 保持不变, "zh" 转换为 "zh-CN", "en-US" 转换为 "en"
    let normalized = if language.starts_with("zh") {
        "zh-CN".to_string()
    } else if language.starts_with("en") {
        "en".to_string()
    } else {
        // 其他语言默认使用英文
        "en".to_string()
    };
    
    Some(normalized)
}

/// 获取当前设置的语言
pub fn current_locale() -> &'static str {
    rust_i18n::locale()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_with_cli_arg() {
        let lang = detect_language(Some("zh-CN"));
        assert_eq!(lang, "zh-CN");
    }

    #[test]
    fn test_detect_default() {
        // 不设置任何参数时，应该返回默认值或系统语言
        let lang = detect_language(None);
        assert!(!lang.is_empty());
    }

    #[test]
    fn test_current_locale() {
        rust_i18n::set_locale("en");
        assert_eq!(current_locale(), "en");
    }
}