# Mindown

[中文版](https://github.com/zyi-ops/Mindown/blob/main/README.md)

**Open-source Knowledge Management Tool Based on Rust and Slint**

> "Rebuild your knowledge network, node by node."

## Introduction
Mindown is developed with Rust and the [Slint](https://slint.dev/) cross-platform GUI framework, deeply integrating mind map visualization with Markdown note-taking. By modeling node relationships, it enables associative knowledge management, providing an efficient solution for developers, researchers, and students to accumulate knowledge.

### Core Features
- **Node-based Knowledge Units**: Each node corresponds to an independent Markdown file, supporting `.md`
- **Bidirectional Interactive Interface**:
    - Draggable and zoomable mind map canvas
    - Real-time rendered Markdown editor
- **Data Persistence**:
    - Topology saved in JSON format
    - Native file system storage for note content
- **Cross-platform Support**: Compatible with Windows/macOS/Linux

## Feature Matrix
| Module         | Description                                 | Status             |
| -------------- | ------------------------------------------- | ------------------ |
| 🗺️ Map Editing  | Node CRUD/parent-child hierarchy/free layout| ✅ Prototype stage  |
| 📝 Note Binding | Link nodes to Markdown files, dual-pane edit| ⚙️ In development   |
| 💾 Persistence  | JSON serialization/deserialization          | 🚧 Architecture     |
| 🔍 Full-text Search | Real-time match for node titles/notes   | 📌 v1.0 target      |
| 📦 Export      | Export mind map as PNG/SVG/PDF               | 🛠️ v1.5 planned     |

## Technical Architecture
```bash
Mindown/
├── Cargo.toml                # Rust project config
├── src/
│   ├── main.rs               # Entry point
│   ├── node/                 # Node data structures & logic
│   ├── markdown/             # Markdown parsing & rendering
│   ├── persistence/          # JSON serialization module
│   └── ui/                   # Slint frontend components
│       ├── main_window.slint # Main window layout
│       └── components/       # Custom UI components
├── resources/                # Static assets (icons/styles)
├── docs/                     # Developer docs & user manual
├── data/                     # Local storage sample data
└── LICENSE                   # GPLv3 license file
```

## Roadmap
### v1.0 Basic Features
- [ ] Node drag-and-drop interaction (with parent-child modeling)
- [ ] Integrated real-time Markdown preview engine
- [ ] JSON structure persistence

### v1.5 Enhanced Features
- [ ] Cross-platform export module (PDF/SVG)
- [ ] Advanced search and filtering
- [ ] Theme switching and custom styles

### v2.0 Ecosystem Expansion
- [ ] Plugin system (Mermaid diagrams/math formulas)
- [ ] Cloud sync (Nextcloud/WebDAV integration)

## Contribution Guide
1. Fork the repo and create a feature branch
     ```bash
     git checkout -b feature/your-feature-name
     ```
2. Implement features and write unit tests
3. Commit using [Conventional Commits](https://www.conventionalcommits.org/) specification
4. Create a Pull Request with detailed description

## License
This project is licensed under the [GNU General Public License v3.0](LICENSE)  
© 2025-present zyi-ops and contributors

## Contact
- Submit Issues for feature requests/bugs
- Email: aarch4082@outlook.com

⭐ Star us to support the next generation of knowledge management tools!