//! 颜色类型定义
//!
//! 本模块定义了所有颜色相关的数据结构。
//!
//! ## 颜色分类
//!
//! - **主色（Primary Color）**：红、黄、蓝，无法由其他颜色混合而成
//! - **副色（Secondary Color）**：橙、绿、紫，由两种主色混合而成
//!
//! ## 示例
//!
//! ```rust
//! use my_art::PrimaryColor;
//!
//! let color = PrimaryColor::Red;
//! ```

/// 主色枚举
///
/// 包含三种基础颜色，是所有颜色的基础。
///
/// # 示例
///
/// ```rust
/// use my_art::PrimaryColor;
///
/// let red = PrimaryColor::Red;
/// let yellow = PrimaryColor::Yellow;
/// let blue = PrimaryColor::Blue;
/// ```
pub enum PrimaryColor {
    /// 红色 — 热情与活力的象征
    Red,
    /// 黄色 — 明亮与温暖的象征
    Yellow,
    /// 蓝色 — 冷静与深邃的象征
    Blue,
}

/// 副色枚举
///
/// 由两种主色混合而成。
///
/// # 混合规则
///
/// | 主色 1 | 主色 2 | 结果 |
/// |--------|--------|------|
/// | Red | Yellow | Orange |
/// | Yellow | Blue | Green |
/// | Blue | Red | Purple |
///
/// # 示例
///
/// ```rust
/// use my_art::SecondaryColor;
///
/// let green = SecondaryColor::Green;
/// assert!(matches!(green, SecondaryColor::Green));
/// ```
#[derive(Debug, PartialEq)]
pub enum SecondaryColor {
    /// 橙色 — Red + Yellow
    Orange,
    /// 绿色 — Yellow + Blue
    Green,
    /// 紫色 — Blue + Red
    Purple,
}
