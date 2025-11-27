# Tauri Plugin Advanced File Manager

[![Cargo Version](https://img.shields.io/crates/v/tauri-plugin-advanced-file-manager.svg)](https://crates.io/crates/tauri-plugin-advanced-file-manager)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

一个强大的 Tauri 文件管理插件，整合了文件系统、对话框和文件打开器的完整功能。

## 功能特性

### 文件系统操作
- 文件和目录的创建、读取、写入、删除
- 文件复制、移动、重命名
- 目录遍历和文件搜索
- 文件权限和元数据获取
- 文件监听 (可选)

### 系统对话框
- 文件/文件夹选择对话框
- 文件保存对话框
- 消息对话框 (信息、警告、错误)
- 确认对话框

### 文件打开器
- 用默认程序打开文件/URL
- 用指定程序打开文件
- 在文件管理器中显示文件

### 跨平台支持
- Windows: 完整支持
- macOS: 完整支持  
- Linux: 完整支持
- Android: 部分支持 (文件系统 + 对话框)
- iOS: 部分支持 (文件系统 + 对话框)

## 安装

### Cargo.toml
```toml
[dependencies]
tauri-plugin-advanced-file-manager = "0.1.0"
```

### src-tauri/Cargo.toml
```toml
[dependencies]
tauri = { version = "2.0", features = ["advanced-file-manager"] }
tauri-plugin-advanced-file-manager = "0.1.0"
```

## 使用方法

### 1. 注册插件 (Rust)

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_advanced_file_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. 前端调用 (TypeScript)

#### 文件系统操作
```typescript
import * as fs from '@tauri-apps/plugin-fs';

// 读取文件
const content = await fs.readTextFile('path/to/file.txt');

// 写入文件
await fs.writeTextFile('path/to/file.txt', 'Hello World!');

// 列出目录
const entries = await fs.readDir('path/to/directory');

// 创建目录
await fs.mkdir('path/to/new/directory');
```

#### 对话框
```typescript
import { dialog } from '@tauri-apps/plugin-dialog';

// 文件选择对话框
const selected = await dialog.open({
  title: '选择文件',
  filters: [
    { name: 'Text', extensions: ['txt', 'md'] },
    { name: 'All', extensions: ['*'] }
  ]
});

// 消息对话框
await dialog.message('操作完成！', { title: '提示', kind: 'info' });

// 确认对话框
const confirmed = await dialog.confirm('确定要删除吗？', { title: '确认' });
```

#### 文件打开器
```typescript
import { opener } from '@tauri-apps/plugin-opener';

// 用默认程序打开文件
await opener.openPath('/path/to/file.txt');

// 用默认浏览器打开URL
await opener.openUrl('https://example.com');

// 在文件管理器中显示文件
await opener.revealItemInDir('/path/to/file.txt');
```

## 权限配置

在 `src-tauri/capabilities/default.json` 中添加权限：

```json
{
  "identifier": "default",
  "windows": ["main"],
  "permissions": [
    "advanced-file-manager:allow-read-file",
    "advanced-file-manager:allow-write-file", 
    "advanced-file-manager:allow-read-dir",
    "advanced-file-manager:allow-create-dir",
    "advanced-file-manager:allow-remove-file",
    "advanced-file-manager:allow-open-dialog",
    "advanced-file-manager:allow-save-dialog",
    "advanced-file-manager:allow-message-dialog",
    "advanced-file-manager:allow-open-path",
    "advanced-file-manager:allow-open-url",
    "advanced-file-manager:allow-reveal-item-in-dir"
  ]
}
```

## 开发

### 构建
```bash
cargo build
```

### 测试
```bash
cargo test
```

### 格式化
```bash
cargo fmt
```

### 检查
```bash
cargo clippy
```

## 许可证

Apache 2.0 License. See [LICENSE](LICENSE) for details.

## 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解更多信息。

## 支持

如果遇到问题，请在 [GitHub Issues](https://github.com/1600822305/tauri-plugin-advanced-file-manager/issues) 中提交.
