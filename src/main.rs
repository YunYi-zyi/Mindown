mod node;
mod persistence;

use node::{Node, MindownData};
use persistence::toml::{load_toml, save_toml};
use std::path::Path;

use crate::node::{add_node, delete_node};

fn main() {
    let path = Path::new("data/example.toml");

    // 从文件中加载数据
    let mut data = load_toml(path).expect("Failed to load data from file");

    // 添加节点
    add_node(&mut data, 4, "第四节点".to_string(), "内容".to_string());
    
    // 删除节点
    add_node(&mut data, 5, "第五节点".to_string(), "内容".to_string());
    delete_node(&mut data, 5);

    // 保存数据
    save_toml(&data, &path).expect("Failed to save files");
}