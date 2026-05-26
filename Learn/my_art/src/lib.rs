//! # My Art
//!
//! 一个用于艺术建模和颜色处理的库。
//!
//! ## 模块概览
//!
//! - [`kinds`] — 定义颜色类型
//! - [`utils`] — 实用工具函数
//!
//! ## 快速上手
//!
//! ```rust
//! use my_art::PrimaryColor;
//! use my_art::mix;
//!
//! let red = PrimaryColor::Red;
//! let blue = PrimaryColor::Blue;
//! let purple = mix(red, blue);
//! ```
//!
//! ## 设计理念
//!
//! 本库遵循"类型安全优先"原则，所有颜色操作都经过类型检查，
//! 避免在运行时出现无效的颜色组合。

// 公开重导出，让用户不需要知道内部模块结构
pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds;
pub mod utils;
