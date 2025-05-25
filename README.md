# Mindown

[English Version](https://github.com/zyi-ops/Mindown/blob/main/README_EN.md)

**基于 Rust 与 Slint 的开源知识管理工具**  

> "以节点为单位，重构你的知识网络"

## 简介  
Mindown 使用 Rust 语言与 [Slint](https://slint.dev/) 跨平台 GUI 框架开发，将思维导图可视化与 Markdown 笔记深度融合。通过节点关系建模实现知识关联性管理，为开发者/研究人员/学生提供高效的知识沉淀解决方案。

### 核心特性  
- **节点化知识单元**：每个节点对应独立 Markdown 文件，支持 `.md` 
- **双向交互界面**：  
  - 可拖拽缩放的思维导图画布  
  - 实时渲染的 Markdown 编辑器  
- **数据持久化**：  
  - JSON 格式保存拓扑结构  
  - 原生文件系统存储笔记内容  
- **跨平台支持**：Windows/macOS/Linux 全平台兼容  

## 功能矩阵  
| 模块         | 功能描述                         | 状态           |
| ------------ | -------------------------------- | -------------- |
| 🗺️ 导图编辑   | 节点增删改查/父子层级/自由布局   | ✅ 原型开发中   |
| 📝 笔记绑定   | 节点关联 Markdown 文件，双屏编辑 | ⚙️ 开发中       |
| 💾 数据持久化 | JSON 结构序列化/反序列化         | 🚧 架构设计     |
| 🔍 全文搜索   | 实时匹配节点标题与笔记内容       | 📌 1.0 版本目标 |
| 📦 导出功能   | PNG/SVG/PDF 格式导出思维导图     | 🛠️ 1.5 版本规划 |

## 技术架构  
```bash
Mindown/
├── Cargo.toml                # Rust 项目配置
├── src/
│   ├── main.rs               # 启动入口
│   ├── node/                 # 节点数据结构与逻辑
│   ├── markdown/             # Markdown 解析与渲染
│   ├── persistence/          # JSON 序列化模块
│   └── ui/                   # Slint 前端组件
│       ├── main_window.slint # 主界面布局
│       └── components/       # 自定义 UI 组件
├── resources/                # 静态资源（图标/样式）
├── docs/                     # 开发文档与用户手册
├── data/                     # 本地存储示例数据
└── LICENSE                   # GPLv3 协议文件
```

## 开发路线图  
### v1.0 基础功能  
- [ ] 节点拖拽交互系统（含父子关系建模）  
- [ ] Markdown 实时预览引擎集成  
- [ ] JSON 结构持久化方案  

### v1.5 增强功能  
- [ ] 跨平台文件导出模块（PDF/SVG）  
- [ ] 高级搜索与过滤系统  
- [ ] 主题切换与自定义样式  

### v2.0 扩展生态  
- [ ] 插件系统（支持 Mermaid 图表/数学公式）  
- [ ] 云同步服务（集成 Nextcloud/WebDAV）  

## 贡献指南  
1. Fork 仓库并创建特性分支  
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. 实现功能并编写单元测试  
3. 提交符合 [Conventional Commits](https://www.conventionalcommits.org/) 规范的 commit  
4. 创建 Pull Request 并附上详细说明  

## 许可证  
本项目采用 [GNU General Public License v3.0](LICENSE)  
© 2025-present zyi-ops 及贡献者  

## 联系方式  
- 提交 Issue 讨论需求/缺陷  
- 邮件联系：aarch4082@outlook.com   

⭐ Star 支持我们，共建下一代知识管理工具！
