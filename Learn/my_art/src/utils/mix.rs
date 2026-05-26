//! 颜色混合算法
//!
//! 实现主色到副色的混合逻辑。
//!
//! ## 算法原理
//!
//! 采用简单的查表法，将两种主色映射为对应的副色。
//! 时间复杂度 O(1)，空间复杂度 O(1)。

use crate::kinds::{PrimaryColor, SecondaryColor};

/// 将两种主色混合为副色
///
/// 根据颜色理论，两两混合三种主色可以得到三种副色。
///
/// # 参数
///
/// * `c1` — 第一种主色
/// * `c2` — 第二种主色
///
/// # 返回值
///
/// 返回混合后的副色
///
/// # 示例
///
/// ```rust
/// use my_art::mix;
/// use my_art::{PrimaryColor, SecondaryColor};
///
/// let orange = mix(PrimaryColor::Red, PrimaryColor::Yellow);
/// assert_eq!(orange, SecondaryColor::Orange);
///
/// let green = mix(PrimaryColor::Yellow, PrimaryColor::Blue);
/// assert_eq!(green, SecondaryColor::Green);
///
/// let purple = mix(PrimaryColor::Blue, PrimaryColor::Red);
/// assert_eq!(purple, SecondaryColor::Purple);
/// ```
///
/// # 注意事项
///
/// 相同颜色混合不会产生新颜色，但目前未对此情况做特殊处理，
/// 会返回 `SecondaryColor::Green`。
///
/// ```rust
/// # use my_art::mix;
/// # use my_art::{PrimaryColor, SecondaryColor};
/// // 不推荐：相同颜色混合
/// let result = mix(PrimaryColor::Red, PrimaryColor::Red);
/// // 结果无意义，请避免
/// ```
pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    match (c1, c2) {
        (PrimaryColor::Red, PrimaryColor::Yellow) | (PrimaryColor::Yellow, PrimaryColor::Red) => {
            SecondaryColor::Orange
        }

        (PrimaryColor::Yellow, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Yellow) => {
            SecondaryColor::Green
        }

        _ => SecondaryColor::Purple,
    }
}
