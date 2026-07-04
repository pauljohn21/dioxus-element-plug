# 🚀 Dioxus Element Plug 极简入门

这是一个最简单的开始方式，只需要 5 分钟就能运行起来！

## 📝 步骤 1: 创建新项目

```bash
# 创建新的 Dioxus 项目
dx create my-element-app
cd my-element-app
```

## 📦 步骤 2: 添加依赖

在 `Cargo.toml` 中添加：

```toml
dioxus = { version = "0.7", features = ["web"] }
dioxus-element-plug = "0.1.2"
```

## 🎨 步骤 3: 修改主组件

编辑 `src/main.rs`：

```rust
use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // 引入 Element Plus CSS
        document::Link {
            rel: "stylesheet",
            href: "//unpkg.com/element-plus/dist/index.css"
        }
        
        div {
            style: "padding: 20px;",
            
            h1 { "我的第一个 Dioxus Element 应用" }
            
            // 按钮组件
            Button {
                variant: ButtonVariant::Primary,
                "点击我！"
            }
            
            Button {
                variant: ButtonVariant::Success,
                "成功按钮"
            }
            
            // 输入框组件  
            Input {
                placeholder: "请输入您的名字...",
            }
            
            // 布局组件
            Container {
                Header {
                    h2 { "页面标题" }
                }
                Main {
                    p { "欢迎来到我的应用！" }
                }
                Footer {
                    p { "版权所有 © 2024" }
                }
            }
        }
    }
}
```

## ▶️ 步骤 4: 运行应用

```bash
# 开发模式运行
dx serve --hot-reload

# 或者
cargo run
```

## 🎉 完成！

现在你可以看到完整样式的 Element UI 组件在你的 Dioxus 应用中运行！

## 📚 接下来可以做什么？

1. **浏览组件**: 查看 [USAGE_GUIDE.md](USAGE_GUIDE.md) 了解所有可用组件
2. **自定义主题**: 学习如何覆盖 CSS 变量来定制颜色和样式
3. **深入学习**: 查看 examples 目录中的完整示例
4. **生产部署**: 学习如何优化和部署你的应用

## 🔧 常用组件示例

### 按钮组件
```rust
Button { "默认按钮" }
Button {
    variant: ButtonVariant::Primary,
    "主要按钮"
}
Button {
    variant: ButtonVariant::Success,
    size: Some(ButtonSize::Large),
    "大型成功按钮"
}
```

### 输入框组件
```rust
Input {
    placeholder: "普通输入框",
}
PasswordInput {
    placeholder: "密码输入框",
}
TextArea {
    placeholder: "多行文本框",
    rows: Some(4),
}
```

### 布局组件
```rust
Container {
    Header { "页面头部" }
    Main { "主要内容" }
    Footer { "页面底部" }
}

Row {
    Col { span: Some(12) { "栅格列" }}
}
```

## 💡 小技巧

1. **热重载**: 使用 `dx serve --hot-reload` 实时预览修改
2. **浏览器调试**: 使用浏览器的开发者工具检查元素和样式
3. **响应式**: 所有组件都天然支持响应式设计
4. **无障碍**: Element UI 组件内置无障碍支持

---

这就是开始所需的一切！从简单的 CSS 链接开始，然后逐步探索更多高级功能。

记住：这已经是一个可以工作的完整应用，只需要 10 行核心代码！ 🎯