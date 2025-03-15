/* 自定义Result错误类型 (总览整个项目) */

// 简化Result类型
pub type Result<T> = std::result::Result<T, Error>;

// 枚举声明
pub enum Error {

}