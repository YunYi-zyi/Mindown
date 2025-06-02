// src/persistence/toml.rs
use crate::node::{MindownData, Node};
use std::fs;
use std::path::Path;
use toml;

/// 从 TOML 文件加载 MindownData
/// 
/// # 参数
/// * `path` - TOML 文件的路径引用
/// 
/// # 返回值
/// * `Result<MindownData, Box<dyn std::error::Error>>` - 成功时返回包含数据的 Ok，失败时返回错误信息
/// 
/// # 流程说明
/// 1. 使用 fs::read_to_string 读取文件内容为字符串
/// 2. 使用 toml::from_str 将 TOML 格式字符串解析为 MindownData 结构体
pub fn load_toml(path: &Path) -> Result<MindownData, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let mindown_data: MindownData = toml::from_str(&data)?;
    Ok(mindown_data)
}

/// 将 MindownData 写入 TOML 文件
/// 
/// # 参数
/// * `data` - 要保存的 MindownData 数据引用
/// * `path` - 目标文件的路径引用
/// 
/// # 返回值
/// * `Result<(), Box<dyn std::error::Error>>` - 成功时返回 Ok(())，失败时返回错误信息
/// 
/// # 流程说明
/// 1. 使用 toml::to_string 将结构体序列化为 TOML 格式字符串
/// 2. 使用 fs::write 将字符串写入指定路径的文件
pub fn save_toml(data: &MindownData, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = toml::to_string(data)?;
    fs::write(path, toml_str)?;
    Ok(())
}
