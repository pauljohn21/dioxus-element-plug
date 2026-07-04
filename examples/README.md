# Dioxus Element Plug 示例项目

现代 Dioxus 0.7+ 最佳实践示例

## ✨ 唯一的现代化示例

### with-scss-asset/ - 推荐的现代 DX 示例

🔥 **现代的开发体验** - 展示 Dioxus Element Plug 的最佳使用方式：

- 🚀 使用内置 manganis SCSS 支持
- ⚡ 零配置设置
- 🔄 热重载开发体验
- 🎨 完整的 Element UI 风格

```bash
cd examples/with-scss-asset
cargo run
```

This is the recommended way to use Dioxus Element Plug for all projects.

## 🎓 快速开始指南

### 方式一：现代 DX (推荐)

```bash
# 1. 进入现代示例目录
cd examples/with-scss-asset

# 2. 查看示例代码
cat src/main.rs

# 3. 运行示例
cargo run

# 4. 编辑 SCSS 文件查看热重载效果
# 修改 assets/theme-chalk.scss 文件
```

### 方式二：传统方式

```bash
# 1. 进入传统示例目录
cd examples/basic

# 2. 运行示例
cargo run

# 3. 需要先编译 CSS (在项目根目录)
cd ../../
make build-css
```

## 📁 项目结构说明

```
examples/
├── README.md              # 本文件 - 示例项目总览
├── with-scss-asset/       # ✅ 现代示例 (推荐)
│   ├── Cargo.toml         # 包含 manganis 依赖
│   ├── README.md          # 详细使用说明
│   ├── assets/
│   │   └── theme-chalk.scss # SCSS 源码文件
│   └── src/
│       └── main.rs        # 演示内置 SCSS 支持
│
└── basic/                 # 🔧 传统示例
    ├── Cargo.toml         # 仅标准依赖
    ├── src/
    │   └── main.rs         # 使用预编译 CSS
    └── public/            # 静态文件目录
        └── dist/
            └── theme-chalk.css  # 编译后的 CSS
```

## 🎯 使用建议

### 什么时候选择现代方式？

选择 **with-scss-asset** 示例当：
- ✅ 新项目，使用 Dioxus 0.7+
- ✅ 希望简化构建流程
- ✅ 偏好 Rust 生态的解决方案
- ✅ 想要热重载支持
- ✅ 不想管理外部构建工具

### 什么时候选择传统方式？

选择 **basic** 示例当：
- ✅ 现有项目迁移
- ✅ 需要灵活的自定义构建选项
- ✅ 团队熟悉传统的 Sass 工具链
- ✅ 项目需要特定的 CSS 后处理
- ✅ 不想添加额外的 Rust 依赖

## 🚀 功能对比

| 功能 | 现代方式 | 传统方式 |
|------|----------|----------|
| 设置复杂度 | ⭐⭐⭐ 很简单 | ⭐⭐ 中等 |
| 构建速度 | ⭐⭐⭐ 很快 | ⭐⭐ 较慢 |
| 热重载 | ⭐⭐⭐ 支持 | ⭐ 有限 |
| 定制化 | ⭐⭐ 有限 | ⭐⭐⭐ 强大 |
| 工具链 | ⭐⭐⭐ 纯 Rust | ⭐⭐ 需要 Node.js |

## 💡 开发技巧

### 现代 DX 开发流程

1. **开始开发**
   ```bash
   cd examples/with-scss-asset
   cargo watch -x run
   ```

2. **修改 SCSS** - 直接编辑 `assets/theme-chalk.scss`

3. **查看变化** - 浏览器自动刷新

### 传统开发流程

1. **开始开发**
   ```bash
   # 终端 1: 监听 SCSS 变化
   cd ../.. && make watch-css
   
   # 终端 2: 运行应用
   cd examples/basic && cargo watch -x run
   ```

2. **修改 SCSS** - 编辑项目根目录的 `scss/` 文件

3. **CSS 自动编译** - 浏览器自动刷新

## 🔧 故障排除

### 现代方式常见问题

**Q: asset 宏找不到资源文件？**
A: 确保 SCSS 文件路径正确，相对于项目根目录。运行 `cargo check` 查看具体错误。

**Q: 热重载不工作？**
A: 确保使用 `cargo watch` 而不是简单的 `cargo run`。

### 传统方式常见问题

**Q: CSS 文件找不到？**
A: 确保先运行了 `make build-css` 编译 SCSS 文件。

**Q: 样式不生效？**
A: 检查浏览器开发者工具中的 Network 面板，确认 CSS 文件加载成功。

## 📚 进一步学习

### 深入学习现代方式
- [Dioxus 官方文档](https://dioxuslabs.com/learn/0.7/)
- [Manganis 使用指南](https://github.com/DioxusLabs/dioxus/tree/main/packages/manganis)
- [SCSS 官方文档](https://sass-lang.com/documentation/)

### 深入学习传统方式  
- [Sass 命令行使用](https://sass-lang.com/documentation/cli)
- [Dioxus 样式指南](../../DIOXUS_STYLING.md)
- [Theme Chalk SCSS 结构](../../SCSS_STRUCTURE.md)

## 🎓 迁移指南

从传统方式迁移到现代方式：

1. **添加依赖**
   ```toml
   [dependencies]
   manganis = { version = "0.7.9", features = ["dioxus"] }
   ```

2. **引入 SCSS**
   ```rust
   use manganis::asset;
   static STYLES: Asset = asset!("/assets/your-styles.scss");
   ```

3. **移除手动 CSS 引用**
   ```diff
   - document::Stylesheet { href: "/dist/theme-chalk.css" }
   ```

4. **删除构建步骤**
   ```diff
   - make build-css  # 不再需要
   ```

## 🎉 结论

Dioxus Element Plug 现在提供了两种强大且灵活的方式来处理 CSS，让开发者可以根据自己的项目需求和团队偏好选择最适合的工作流。无论选择哪种方式，都能享受到完整的 Element UI 组件和样式支持！

**新项目的开发者强烈推荐使用现代 DX (`with-scss-asset`)，享受最流畅的开发体验！**
