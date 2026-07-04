# Dioxus 0.7 样式支持与 SCSS 集成指南

## Dioxus 0.7 样式架构概述

Dioxus 0.7 采用了现代化的 Web 框架样式处理方式，**现在支持内置的 SCSS 编译！**

### 🔧 当前的样式支持状态
- ✅ **CSS 文件支持**: 完全支持通过 `<link>` 标签引入外部 CSS 文件
- ✅ **内联样式**: 支持通过 `style` 属性设置内联样式
- ✅ **CSS Modules**: 支持模块化的 CSS 处理
- 🎉 **SCSS/SASS 内置支持**: 通过 **manganis asset 宏**实现内置 SCSS 编译

### 📦 可选的工作流

Dioxus 0.7 现在提供了两种 SCSS 处理方式：

#### 🚀 方式一：Dioxus 内置 SCSS 支持 (推荐用于新项目)

1. **开发阶段**: 编写 SCSS 源文件
2. **声明阶段**: 使用 `asset!()` 宏声明 SCSS 资源
3. **构建阶段**: Dioxus 自动编译 SCSS 并注入到应用中

```rust
use manganis::*;
static STYLES: Asset = asset!("/assets/theme-chalk.scss");
```

#### 🔧 方式二：传统外部编译方式 (推荐用于现有项目)

1. **开发阶段**: 编写 SCSS 源文件
2. **构建阶段**: 使用外部 Sass 编译器转换为 CSS 
3. **运行阶段**: Dioxus 应用加载编译后的 CSS 文件

```bash
make build-css  # 使用 Makefile 编译
```

## 构建流程详解

### 1. SCSS 源文件组织

本项目采用标准的 Element UI theme-chalk SCSS 结构：

```
scss/
├── index.scss              # 主入口文件
├── common/
│   ├── var.scss           # 变量定义
│   └── mixins.scss        # 混合宏
├── mixins/
│   └── _button.scss       # 按钮相关混合
└── packages/
    ├── button/
    │   └── src/
    │       └── index.scss # 按钮组件样式
    └── ...
```

### 2. 自动化构建配置

#### 方式一：Dioxus 内置 SCSS 编译
无需外部构建工具，直接使用 asset 宏：

```rust
// 在 Rust 代码中
use manganis::*;
static THEME_STYLES: Asset = asset!("/assets/theme-chalk.scss");
```

#### 方式二：传统 Makefile 构建

```makefile
# 安装依赖
make setup

# 编译 CSS
make build-css

# 开发模式（监听文件变化）
make watch-css
```

### 3. 与 Dioxus 集成

#### 方式一：使用内置 SCSS 支持 (推荐)

```rust
use dioxus::prelude::*;
use manganis::*;

// 声明 SCSS 资源
static STYLES: Asset = asset!("/assets/theme-chalk.scss");

#[component]
fn App() -> Element {
    rsx! {
        // Manganis 会自动处理 SCSS 编译和注入
        div {
            class: "el-button el-button--primary",
            "Primary Button"
        }
    }
}
```

#### 方式二：使用预编译 CSS 文件

```rust
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        // 引入编译后的 CSS 文件
        document::Stylesheet {
            href: "/dist/theme-chalk.css"
        }
        
        div {
            class: "el-button el-button--primary",
            "Primary Button"
        }
    }
}
```

## 主题定制

### 修改变量

编辑 `scss/common/var.scss` 文件来自定义主题：

```scss
// 色彩系统
$--color-primary: #409EFF;
$--color-success: #67C23A;
$--color-warning: #E6A23C;
$--color-danger: #F56C6C;
$--color-info: #909399;

// 字体
$--font-size-base: 14px;
$--font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", Arial, sans-serif;

// 边框
$--border-radius-base: 4px;
$--border-width-base: 1px;
```

### 组件级定制

可以针对特定组件进行样式覆盖：

```scss
// 定制按钮样式
.el-button {
    &--large {
        height: 40px;
        padding: 0 20px;
        font-size: 16px;
    }
    
    &--small {
        height: 24px;
        padding: 0 8px;
        font-size: 12px;
    }
}
```

## 开发最佳实践

### 1. 样式隔离

使用 CSS Modules 或作用域选择器避免样式冲突：

```rust
// 使用局部类名
#[component]
fn CustomButton() -> Element {
    rsx! {
        button {
            class: "my-app-button",
            // 避免直接使用 el-button 类名
            style: "/* 自定义样式 */",
            "Custom Button"
        }
    }
}
```

### 2. 响应式设计

结合 theme-chalk 的响应式工具类：

```rust
div {
    class: "el-row",
    div {
        class: "el-col-24 el-col-md-12 el-col-lg-8",
        "Responsive column"
    }
}
```

### 3. 性能优化

- 使用压缩的 CSS 文件 (`--style=compressed`)
- 按需引入组件样式
- 使用 CSS 变量进行运行时主题切换

## 常见问题解答

### Q: Dioxus 现在支持内置 SCSS 了吗？

A: ✅ **是的！** 从 Dioxus 0.7 开始，通过 manganis 提供的 asset 宏，你可以直接在 Rust 代码中引入 SCSS 文件：

```rust
use manganis::*;
static STYLES: Asset = asset!("/assets/your-styles.scss");
```

Dioxus 会在构建时自动编译 SCSS 文件，无需外部构建工具。

### Q: 两种方式有什么区别？我应该选择哪种？

A: 

**内置 SCSS 支持 (manganis)** 优势：
- ✅ 无需外部构建工具
- ✅ 自动编译和优化
- ✅ 更好的开发体验（热重载）
- ✅ 资源自动捆绑

**传统外部编译方式** 优势：
- ✅ 适合现有项目迁移
- ✅ 更灵活的构建配置
- ✅ 与现有 Sass 工具链兼容

**推荐：** 新项目使用内置 SCSS 支持，现有项目可以继续使用传统方式。

### Q: 如何在开发过程中实现热重载？

A: 使用 `make watch-css` 监听 SCSS 文件变化，配合 Dioxus 的热重载功能：

```bash
# 终端 1: 监听 CSS 变化
make watch-css

# 终端 2: 启动 Dioxus 开发服务器
dx serve --hot-reload
```

### Q: 是否支持 CSS-in-RS 方案？

A: 可以通过 `style` 属性实现，但不推荐用于复杂样式。建议：
- 简单样式：使用内联 `style` 属性
- 复杂样式：使用预编译的 SCSS/CSS 文件
- 动态样式：结合 CSS 变量和 Rust 状态

## 参考资源

- [Dioxus 官方样式文档](https://dioxuslabs.com/learn/0.7/essentials/ui/styling)
- [Element UI theme-chalk 源码](https://github.com/ElemeFE/element/tree/dev/packages/theme-chalk)
- [Sass 官方文档](https://sass-lang.com/documentation/)

---
> **重要提示**: 虽然 Dioxus 0.7 不支持内置 SCSS 编译，但这种架构分离实际上更符合现代前端工程化实践，提供了更大的灵活性和性能优化空间。
