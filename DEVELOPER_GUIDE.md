# Dioxus Element Plug - 开发者指南

## 🎯 项目概述

**Dioxus Element Plug** 是一个为 Dioxus 0.7+ 应用提供 Element UI theme-chalk 样式的现代化组件库。

使用 Dioxus 内置的 manganis 实现零配置 SCSS 开发体验！

## 🚀 快速入门

### 方式一：现代 DX (推荐)

```bash
git clone https://github.com/pauljohn21/dioxus-element-plug.git
cd dioxus-element-plug

# 1. 查看现代示例
cd examples/with-scss-asset

# 2. 运行示例
cargo run

# 3. 开始开发 - 修改 assets/theme-chalk.scss 文件
```

### 方式二：传统方式

```bash
git clone https://github.com/pauljohn21/dioxus-element-plug.git
cd dioxus-element-plug

# 1. 编译 CSS
make traditional
make build-css

# 2. 查看传统示例
cd examples/basic
cargo run
```

## 🏗️ 项目架构

### 核心库结构

```
src/
├── lib.rs        # 主库入口，包含两种 CSS 方式的使用说明
├── components/   # UI 组件实现
│   ├── button.rs
│   ├── input.rs
│   └── layout.rs
├── theme.rs      # 传统主题的常量和工具
└── scss.rs       # manganis SCSS 支持 (条件编译)

scss/             # SCSS 源文件 (传统方式使用)
├── index.scss
├── components/
├── layout/
└── mixins/

examples/
├── README.md     # 示例选择指南
├── with-scss-asset/ # 现代 DX 示例
└── basic/        # 传统方式示例
```

### 特性系统

- `default` - 无特殊特性
- `web` - Web 平台支持
- `server` - 服务端渲染支持
- `manganis` - 启 conditionally compiled SCSS 支持

## 🎨 使用示例

### 现代方式 - 内置 SCSS

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;
use dioxus_element_plug::scss::asset;

// 声明 SCSS 资源 - Dioxus 自动编译
static STYLES: Asset = asset!("/assets/theme-chalk.scss");

#[component]
fn App() -> Element {
    rsx! {
        div {
            // 直接使用组件
            Button {
                variant: ButtonVariant::Primary,
                "Primary Button"
            }
            
            Input {
                placeholder: "Enter text...",
                size: Some(InputSize::Medium),
            }
        }
    }
}
```

### 传统方式 - 预编译 CSS

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        // 手动引入编译后的 CSS
        document::Stylesheet {
            href: "/dist/theme-chalk.css"
        }
        
        div {
            Button {
                variant: ButtonVariant::Primary,
                "Primary Button"
            }
        }
    }
}
```

## 🛠️ 开发工具

### Makefile 工具

```bash
# 显示帮助和选择指南
make help

# 现代 DX 指南
make modern

# 传统方式设置
make traditional

# 传统 CSS 编译
make build-css     # 编译一次
make watch-css     # 监听变化
make setup         # 安装依赖
```

### Cargo 工具

```bash
# 检查主库
cargo check

# 检查启用 manganis 的版本
cargo check --features manganis

# 运行测试
cargo test

# 构建文档
cargo doc --open
```

## 📦 组件 API

### 按钮组件

```rust
use dioxus_element_plug::components::button::{Button, ButtonVariant, ButtonSize};

rsx! {
    div {
        // 基础按钮
        Button {
            variant: ButtonVariant::Primary,
            on_click: move |_| println!("Clicked!"),
            "Primary Button"
        }
        
        // 不同尺寸的按钮
        Button {
            variant: ButtonVariant::Success,
            size: Some(ButtonSize::Large),
            "Large Success"
        }
        
        // 轮廓按钮
        OutlineButton {
            variant: ButtonVariant::Danger,
            "Danger Outline"
        }
    }
}
```

### 输入框组件

```rust
use dioxus_element_plug::components::input::{Input, InputSize, InputType, PasswordInput};

rsx! {
    div {
        // 文本输入框
        Input {
            input_type: InputType::Text,
            placeholder: "Enter your name...",
            size: Some(InputSize::Medium),
            on_input: move |value| println!("Input: {}", value),
        }
        
        // 密码输入框
        PasswordInput {
            placeholder: "Enter password...",
            clearable: true,
        }
        
        // 搜索输入框
        SearchInput {
            placeholder: "Search...",
            prefix_icon: Some("el-icon-search".to_string()),
        }
    }
}
```

### 布局组件

```rust
use dioxus_element_plug::components::layout::*;

rsx! {
    Container {
        Header {
            height: 80,
            div { "App Header" }
        }
        
        Main {
            Row {
                Col { span: Some(8) { "Sidebar" }}
                Col { span: Some(16) { "Main Content" }}
            }
        }
        
        Footer {
            height: 60,
            div { "App Footer" }
        }
    }
}
```

## 🎨 主题定制

### 现代方式 - 动态 SCSS 变量

```scss
// assets/custom-theme.scss
$primary-color: #ff6b6b;
$success-color: #51cf66;
$warning-color: #ffd43b;
$danger-color: #ff5555;

// 导入基础主题
@import "../../scss/index.scss";
```

然后在 Rust 中引用：

```rust
static CUSTOM_THEME: Asset = asset!("/assets/custom-theme.scss");
```

### 传统方式 - CSS 变量覆盖

```css
/* custom-overrides.css */
:root {
    --el-color-primary: #ff6b6b;
    --el-color-success: #51cf66;
    --el-color-warning: #ffd43b;
    --el-color-danger: #ff5555;
}
```

然后在 HTML 中引入：

```html
<link rel="stylesheet" href="/custom-overrides.css">
<link rel="stylesheet" href="/dist/theme-chalk.css">
```

## 🔧 开发工作流

### 现代 DX 工作流

1. **初始化项目**
   ```bash
   cd your-dioxus-project
   ```

2. **添加依赖**
   ```toml
   [dependencies]
   dioxus-element-plug = "0.1.0"
   manganis = { version = "0.7.9", features = ["dioxus"] }
   ```

3. **创建 SCSS 文件**
   ```scss
   // assets/theme-chalk.scss
   @import "path/to/dioxus-element-plug/scss/index.scss";
   ```

4. **在代码中引用**
   ```rust
   static STYLES: Asset = asset!("/assets/theme-chalk.scss");
   ```

5. **开始开发**
   ```bash
   cargo watch -x run
   ```

### 传统工作流

1. **编译 CSS**
   ```bash
   cd dioxus-element-plug
   make setup
   make build-css
   ```

2. **复制 CSS 到项目**
   ```bash
   cp dist/theme-chalk.css /path/to/your/project/assets/
   ```

3. **在 HTML 中引入**
   ```html
   <link rel="stylesheet" href="/assets/theme-chalk.css">
   ```

4. **开发时监听变化**
   ```bash
   # 终端 1
   cd dioxus-element-plug && make watch-css
   
   # 终端 2
   cargo watch -x run
   ```

## 🚀 性能优化

### 生产构建

```bash
# 现代方式
cargo build --release --features manganis

# 传统方式
make build-css  # 使用压缩选项
cargo build --release
```

### 按需加载

```scss
// 只导入需要的组件
@import "packages/button/src/index.scss";
@import "packages/input/src/index.scss";
// 不导入未使用的组件
```

## 🐛 故障排除

### 常见问题

**Q: CSS 样式不生效？**
A:
- 现代方式：确保 manganis 依赖已添加，SCSS 文件路径正确
- 传统方式：确保 `make build-css` 已成功运行

**Q: 热重载不工作？**
A:
- 使用 `cargo watch -x run` 而不是 `cargo run`
- 确保文件保存后重新编译

**Q: 生产环境样式问题？**
A:
- 现代方式：在生产构建前运行 `cargo clean`
- 传统方式：确保使用压缩版本 CSS

## 📚 参考资源

### 官方文档
- [Dioxus 官方文档](https://dioxuslabs.com/learn/0.7/)
- [Manganis 文档](https://github.com/DioxusLabs/dioxus/tree/main/packages/manganis)
- [Sass 官方文档](https://sass-lang.com/documentation/)
- [Element UI 文档](https://element.eleme.io/)

### 项目文档
- [README.md](README.md) - 项目概述和快速开始
- [QUICKSTART.md](QUICKSTART.md) - 详细开始指南
- [DIOXUS_STYLING.md](DIOXUS_STYLING.md) - 样式处理完整指南
- [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) - 项目结构说明
- [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) - 本文件

## 🎉 结论

Dioxus Element Plug 现在提供了前所未有的灵活性和现代化开发体验。无论您是：

- 🌟 **新项目开发者** - 选择现代 DX，享受零配置的开发体验
- 🏢 **现有项目维护者** - 可以平滑迁移，保持向后兼容
- 👥 **团队协作** - 两种方式都支持，团队可以根据偏好选择

都能够在 Dioxus 应用中获得完整的 Element UI 组件和样式支持！

**开启你的现代化 Dioxus 开发之旅吧！** 🚀
