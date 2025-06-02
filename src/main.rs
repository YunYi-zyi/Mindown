mod node;
mod persistence;

use node::{Node, MindownData};
use persistence::toml::{load_toml, save_toml};
use std::path::Path;

fn main() {
    let path = Path::new("data/example.toml");

    // 从文件中加载数据
    let mut data = load_toml(path).expect("Failed to load data from file");

    // 添加新节点
    data.nodes.push(Node { id: 4, title: "新节点".to_string(), children: None, content: "# 新内容".to_string(), }
    );

    // 保存数据
    save_toml(&data, &path).expect("Failed to save files");
}