// 节点相关操作
// src/node/mod.rs
use serde::{Deserialize, Serialize};

/// 表示思维导图中的一个节点
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    /// 节点的唯一标识符
    pub id: u64,

    /// 节点显示的标题
    pub title: String,

    /// 子节点的ID列表，使用Option<Vec<u64>>来表示可能没有子节点的情况
    pub children: Option<Vec<u64>>,

    /// 节点关联的内容信息
    pub content: String,
}

/// 表示整个思维导图的数据结构
#[derive(Serialize, Deserialize, Debug)]
pub struct MindownData {
    /// 包含所有节点的向量
    pub nodes: Vec<Node>,
}

impl Default for MindownData {
    fn default() -> Self {
        MindownData { nodes: Vec::new() }
    }
}

pub fn add_node(data: &mut MindownData, id: u64, title: String, content: String) {
    data.nodes.push(Node {
        id: id,
        title: title,
        content: content,
        children: None,
    });
}

pub fn delete_node(data: &mut MindownData, id: u64) {
    data.nodes.retain(|node| node.id != id);
}
