# Dioxus Element Plug 使用指南

## 📋 概述

Dioxus Element Plug 提供了 Element UI theme-chalk 的完整组件库。从 v0.1.2 开始，我们移除了有问题的 SCSS 预置宏，为用户提供了更稳定和灵活的使用方式。

## 🚀 快速开始

### 方式 1: 使用预编译 CSS (推荐)

这是最简单和最稳定的方法，适合大多数用户：

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn App() -> Element {
    rsx! {
        // 引入预编译的 Element UI CSS
        document::Link {
            rel: "stylesheet",
            href: "//unpkg.com/element-plus/dist/index.css"
        }
        
        div {
            Button {
                variant: ButtonVariant::Primary,
                "Primary Button"
            }
            
            Input {
                placeholder: "输入文本...",
                size: Some(InputSize::Medium),
            }
        }
    }
}
```

### 方式 2: 使用自定义 SCSS 文件

如果你想自定义主题或只包含需要的组件：

1. 在你的项目中创建 SCSS 文件：

```scss
// assets/theme.scss
@import "node_modules/dioxus-element-plug/scss/index.scss";

// 或只引入需要的组件
@import "node_modules/dioxus-element-plug/scss/components/button.scss";
@import "node_modules/dioxus-element-plug/scss/components/input.scss";
```

2. 在 Rust 代码中使用：

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;
use manganis::asset;

static CUSTOM_STYLES: Asset = asset!("/assets/theme.scss");

fn App() -> Element {
    rsx! {
        div {
            Button {
                variant: ButtonVariant::Success,
                "Custom Themed Button"
            }
        }
    }
}
```

### 方式 3: 复制 SCSS 到项目本地

对于更复杂的自定义需求：

1. 从库的 `scss/` 目录复制文件到你的项目
2. 根据需要修改这些文件
3. 使用 asset 宏引用它们：

```rust
use manganis::asset;

static LOCAL_THEME: Asset = asset!("/scss/local-theme.scss");
```

## 🎨 组件使用示例

### Button 组件

```rust
use dioxus_element_plug::components::button::*;

fn MyButtons() -> Element {
    rsx! {
        div {
            // 基础按钮
            Button { "Default" }
            
            // 主要按钮
            Button {
                variant: ButtonVariant::Primary,
                "Primary Button"
            }
            
            // 不同尺寸
            Button {
                variant: ButtonVariant::Success,
                size: Some(ButtonSize::Large),
                "Large Success Button"
            }
            
            // 禁用状态
            Button {
                variant: ButtonVariant::Warning,
                disabled: true,
                "Disabled Button"
            }
        }
    }
}
```

### Input 组件

```rust
use dioxus_element_plug::components::input::*;

fn MyInputs() -> Element {
    rsx! {
        div {
            Input {
                placeholder: "请输入用户名",
                size: Some(InputSize::Medium),
            }
            
            PasswordInput {
                placeholder: "请输入密码",
            }
            
            TextArea {
                placeholder: "多行文本输入",
                rows: Some(4),
            }
        }
    }
}
```

### Layout 组件

```rust
use dioxus_element_plug::components::layout::*;

fn MyLayout() -> Element {
    rsx! {
        Container {
            Header {
                h1 { "应用标题" }
            }
            
            Main {
                Row {
                    Col { span: Some(12) {
                        p { "主要内容区域" }
                    }}
                }
            }
            
            Footer {
                p { "页脚信息" }
            }
        }
    }
}
```

## 🎯 主题定制

### 使用 CSS 变量覆盖

```css
/* 在你的 CSS 文件中 */
:root {
    --el-color-primary: #ff6b35;
    --el-color-success: #52c41a;
    --el-color-warning: #faad14;
    --el-border-radius-base: 6px;
}
```

### 使用 SCSS 变量覆盖

```scss
// 在你的 SCSS 文件中，在导入 dioxus-element-plug 之前定义变量
$el-color-primary: #ff6b35;
$el-color-success: #52c41a;
$el-border-radius-base: 6px;

@import "dioxus-element-plug/scss/index.scss";
```

## 📚 API 参考

### 导入方式

```rust
// 推荐：使用 prelude 导入常用组件
use dioxus_element_plug::prelude::*;

// 或选择性导入
use dioxus_element_plug::components::button::*;
use dioxus_element_plug::components::input::*;
use dioxus_element_plug::theme::*;
use dioxus_element_plug::scss::{asset, class_names, helpers};
```

### 主题常量

```rust
use dioxus_element_plug::theme::colors;
use dioxus_element_plug::scss::theme_variables;

// 可以直接在 Rust 代码中使用这些常量
let primary_color = colors::PRIMARY; // "#409EFF"
```

### CSS 类名

```rust
use dioxus_element_plug::scss::class_names::button;

// 获取按钮类名
let button_class = button::PRIMARY; // "el-button--primary"
```

## 🔧 故障排除

### SCSS 文件找不到

**问题**: 使用 `asset!()` 宏时出现 "file not found" 错误

**解决方案**: 
- 确保文件路径相对于你的项目根目录
- 或者使用预编译 CSS 方式
- 或者将 SCSS 文件复制到你的项目中

### 样式不生效

**问题**: 组件渲染但没有应用样式

**解决方案**:
- 确认正确引入了 CSS 文件
- 检查浏览器开发者工具的 Network 面板确认 CSS 加载成功
- 确保没有其他 CSS 覆盖了 Element UI 样式

### 构建错误

**问题**: 编译时出现宏相关的错误

**解决方案**:
- 检查是否使用了已移除的预置宏（如 `theme_chalk_scss!()`)
- 改用 `manganis::asset!()` 宏
- 或参考示例使用预编译 CSS

## 📖 更多资源

- [Dioxus 官方文档](https://dioxuslabs.com/learn/0.7/)
- [Manganis 使用指南](https://github.com/DioxusLabs/dioxus/tree/main/packages/manganis)
- [Element Plus 文档](https://element-plus.org/)
- [SCSS 官方文档](https://sass-lang.com/documentation/)

## 🎉 最佳实践

1. **新项目推荐**: 使用预编译 CSS 方式，简单稳定
2. **深度定制**: 使用自定义 SCSS，完全控制样式
3. **性能优化**: 只引入需要的组件样式
4. **主题开发**: 结合 CSS 变量和 SCSS 变量进行定制
5. **版本管理**: 使用具体的版本号避免意外更新

---

**记住**: 从 v0.1.2 开始，我们专注于提供最稳定和灵活的组件库体验！无论是简单的预编译 CSS 还是复杂的自定义主题，都能找到适合你项目的解决方案。