//! # second_release_cfg_crates_io
//! 
//! `second_release_cfg_crates_io` is a collection of utilities to make performing certain
//! calculations more convenient

// 上面的注释打包进文档后, 就处于最上方, 用于描述当前 crate

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let ars = 5;
/// let answer = second_release_cfg_crates_io::add_one(ars);
/// assert_eq!(6, answer);
/// ```
// * 使用cargo doc打包文档注释
pub fn add_one(x: i32) -> i32 {
  return x + 1;
}