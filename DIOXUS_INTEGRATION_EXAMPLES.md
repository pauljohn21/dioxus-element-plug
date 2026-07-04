# Dioxus 0.7 + Theme Chalk 集成示例

本文档展示了如何在 Dioxus 0.7 应用中集成和使用 theme-chalk 组件库。

## 基础集成

### 1. 项目配置

首先确保你的 `Cargo.toml` 配置正确：

```toml
[dependencies]
dioxus = "0.7"
dioxus-element-plug = { path = "../dioxus-element-plug" }
```

### 2. 构建 SCSS

在开发前编译 CSS 文件：

```bash
cd dioxus-element-plug
make setup      # 安装 Sass 编译器
make build-css  # 编译 SCSS 到 CSS
```

### 3. 基本应用结构

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // 引入 theme-chalk CSS (编译后的)
        document::Stylesheet {
            href: "/dist/theme-chalk.css"
        }
        
        div { 
            style: "padding: 20px;",
            
            h1 { "Dioxus + Theme Chalk" }
            
            // 使用 theme-chalk 组件
            Button {
                variant: ButtonVariant::Primary,
                "Primary Button"
            }
        }
    }
}
```

## 组件使用示例

### 按钮组件

```rust
#[component]
fn ButtonExamples() -> Element {
    rsx! {
        div {
            style: "display: flex; gap: 10px; flex-wrap: wrap;",
            
            // 基础按钮
            Button {
                variant: ButtonVariant::Default,
                "Default"
            }
            
            Button {
                variant: ButtonVariant::Primary,
                "Primary"
            }
            
            Button {
                variant: ButtonVariant::Success,
                "Success"
            }
            
            Button {
                variant: ButtonVariant::Warning,
                "Warning"
            }
            
            Button {
                variant: ButtonVariant::Danger,
                "Danger"
            }
            
            Button {
                variant: ButtonVariant::Info,
                "Info"
            }
        }
        
        div {
            style: "margin-top: 20px;",
            
            // 不同尺寸
            Button {
                variant: ButtonVariant::Primary,
                size: ButtonSize::Large,
                "Large"
            }
            
            Button {
                variant: ButtonVariant::Primary,
                size: ButtonSize::Medium,
                "Medium"
            }
            
            Button {
                variant: ButtonVariant::Primary,
                size: ButtonSize::Small,
                "Small"
            }
        }
    }
}
```

### 输入框组件

```rust
use std::rc::Rc;

#[component]
fn InputExamples() -> Element {
    let mut text = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 15px;",
            
            // 基础输入框
            Input {
                placeholder: "请输入内容...",
                value: text,
                on_input: move |value| text.set(value),
            }
            
            // 密码输入框
            PasswordInput {
                placeholder: "请输入密码...",
                value: password,
                on_input: move |value| password.set(value),
                clearable: true,
            }
            
            // 带图标的输入框
            InputWithIcon {
                placeholder: "搜索...",
                icon: "el-icon-search",
                position: IconPosition::Suffix,
            }
        }
    }
}
```

### 布局组件

```rust
#[component]
fn LayoutExamples() -> Element {
    rsx! {
        Container {
            // 头部
            Header {
                height: 80,
                div {
                    style: "padding: 20px; background: #409EFF; color: white;",
                    h2 { "应用头部" }
                }
            }
            
            // 主体内容
            Main {
                Row {
                    Col {
                        span: Some(8),
                        div {
                            style: "padding: 20px; background: #f5f7fa; margin: 10px; border-radius: 4px;",
                            h3 { "左侧栏" }
                            p { "占比 8/24" }
                        }
                    }
                    
                    Col {
                        span: Some(16),
                        div {
                            style: "padding: 20px; background: #fff; margin: 10px; border: 1px solid #dcdfe6;",
                            h3 { "主要内容区域" }
                            p { "占比 16/24" }
                            
                            // 嵌套栅格
                            Row {
                                Col {
                                    span: Some(12),
                                    div {
                                        style: "padding: 10px; background: #ecf5ff;",
                                        "嵌套列 1"
                                    }
                                }
                                Col {
                                    span: Some(12),
                                    div {
                                        style: "padding: 10px; background: #f0f9ff;",
                                        "嵌套列 2"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 底部
            Footer {
                height: 60,
                div {
                    style: "padding: 20px; background: #333; color: white; text-align: center;",
                    "© 2024 Dioxus Theme Chalk"
                }
            }
        }
    }
}
```

## 主题定制示例

### 动态主题切换

```rust
use dioxus::prelude::*;

#[component]
fn ThemeSwitcher() -> Element {
    let mut theme = use_signal(|| "light".to_string());
    
    let theme_styles = match theme()().as_str() {
        "dark" => "background: #303133; color: #fff;",
        _ => "background: #fff; color: #303133;",
    };
    
    rsx! {
        div {
            style: "padding: 20px;",
            
            h2 { "主题切换示例" }
            
            div {
                style: "margin-bottom: 20px;",
                
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: move |_| theme.set("light".to_string()),
                    "浅色主题"
                }
                
                Button {
                    variant: ButtonVariant::Info,
                    on_click: move |_| theme.set("dark".to_string()),
                    "深色主题"
                }
            }
            
            div {
                style: "{theme_styles} padding: 20px; border-radius: 4px;",
                
                h3 { "预览区域" }
                p { "当前主题: {theme}" }
                
                Button {
                    variant: ButtonVariant::Success,
                    "主题化按钮"
                }
            }
        }
    }
}
```

### CSS 变量定制

你也可以通过 CSS 变量进行实时主题定制：

```rust
#[component]
fn CssVariableTheming() -> Element {
    let mut primary_color = use_signal(|| "#409EFF".to_string());
    
    rsx! {
        div {
            // 通过 CSS 变量定义主题色
            style: format!(":root {{ --el-color-primary: {}; }}", primary_color()()),
            
            div {
                style: "padding: 20px;",
                
                h2 { "CSS 变量主题定制" }
                
                // 颜色选择器
                div {
                    style: "margin: 20px 0;",
                    
                    label { "主色调: " }
                    input {
                        r#type: "color",
                        value: primary_color,
                        on_input: move |color| primary_color.set(color),
                    }
                }
                
                // 使用自定义主题色的按钮
                div {
                    style: "margin-top: 20px;",
                    
                    Button {
                        variant: ButtonVariant::Primary,
                        style: format!("background-color: {}; border-color: {};", 
                                     primary_color()(), primary_color()()),
                        "自定义颜色按钮"
                    }
                }
            }
        }
    }
}
```

## 响应式设计

### 使用 theme-chalk 的响应式工具类

```rust
#[component]
fn ResponsiveLayout() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",
            
            h2 { "响应式布局示例" }
            
            // 移动优先的响应式网格
            Row {
                Col {
                    // 移动端：24列，平板：12列，桌面：8列
                    span: Some(24),
                    class: "el-col-md-12 el-col-lg-8",
                    div {
                        style: "padding: 20px; background: #409EFF; color: white; margin: 5px;",
                        h3 { "响应式卡片 1" }
                        p { "在移动设备上占满宽度，平板设备上占一半，桌面设备上占三分之一" }
                    }
                }
                
                Col {
                    span: Some(24),
                    class: "el-col-md-12 el-col-lg-8",
                    div {
                        style: "padding: 20px; background: #67C23A; color: white; margin: 5px;",
                        h3 { "响应式卡片 2" }
                        p { "自适应不同屏幕尺寸" }
                    }
                }
                
                Col {
                    span: Some(24),
                    class: "el-col-md-12 el-col-lg-8",
                    div {
                        style: "padding: 20px; background: #E6A23C; color: white; margin: 5px;",
                        h3 { "响应式卡片 3" }
                        p { "完美的响应式体验" }
                    }
                }
            }
        }
    }
}
```

## 性能优化建议

### 1. CSS 文件优化

```bash
# 生产环境使用压缩版本
sass scss/index.scss dist/theme-chalk.min.css --style=compressed
```

然后在应用中引用压缩版本：

```rust
document::Stylesheet {
    href: "/dist/theme-chalk.min.css"
}
```

### 2. 按需加载

对于大型应用，可以按需编译和加载组件样式：

```scss
// custom-bundle.scss
@import "packages/button/src/index.scss";
@import "packages/input/src/index.scss";
// 只包含需要的组件
```

### 3. 服务端渲染 (SSR) 优化

```rust
#[component]
fn App() -> Element {
    rsx! {
        // 在 SSR 模式下预加载 CSS
        document::Link {
            rel: "preload",
            href: "/dist/theme-chalk.css",
            r#as: "style",
        }
        
        document::Stylesheet {
            href: "/dist/theme-chalk.css"
        }
        
        // 应用内容
        MainContent {}
    }
}
```

## 故障排除

### 常见问题

1. **样式不生效**
   - 确保已经运行 `make build-css` 编译了最新的 CSS
   - 检查浏览器开发者工具中的网络请求是否成功加载 CSS 文件
   - 确认没有缓存问题（可以尝试强制刷新）

2. **组件样式错乱**
   - 检查是否有其他 CSS 文件覆盖了 theme-chalk 的样式
   - 使用浏览器的开发者工具检查元素的最终样式
   - 确保 HTML 结构符合 theme-chalk 的要求

3. **构建错误**
   - 确保 Sass 编译器版本兼容（建议 Dart Sass 1.77+）
   - 检查 SCSS 语法是否正确
   - 确认所有依赖文件路径正确

### 调试技巧

```rust
// 添加调试信息
#[component]
fn DebugComponent() -> Element {
    rsx! {
        div {
            // 添加临时样式辅助调试
            style: "border: 1px solid red;",
            
            // 使用浏览器开发者工具检查
            button {
                class: "el-button el-button--primary",
                style: "background-color: red !important;", // 强制样式用于测试
                "调试按钮"
            }
        }
    }
}
```

通过这些示例，你可以快速开始在 Dioxus 0.7 项目中使用 theme-chalk 组件库，并充分利用现代的 CSS 处理工作流程。
