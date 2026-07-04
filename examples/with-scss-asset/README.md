# Dioxus 0.7 内置 SCSS 支持示例

这个示例展示了如何在 Dioxus 0.7 应用中直接使用 SCSS 文件，利用 manganis 提供的内置 SCSS 支持。

## 🚀 新的 Dioxus 0.7 SCSS 支持特性

Dioxus 0.7 通过 manganis crate 提供了内置的 SCSS 编译支持，这意味着：

- ✅ **无需外部构建工具**: 不需要单独的 Sass 编译器
- ✅ **自动编译**: SCSS 文件会在应用构建时自动编译为 CSS
- ✅ **资源优化**: 编译后的 CSS 会自动被优化和捆绑
- ✅ **开发体验提升**: 支持热重载，开发时修改 SCSS 文件会自动更新

## 📦 项目结构

```
with-scss-asset/
├── src/main.rs          # 示例应用
├── assets/
│   └── theme-chalk.scss # SCSS 样式文件
└── Cargo.toml           # 依赖配置
```

## 🔧 关键技术栈

- **Dioxus 0.7**: 现代化的 Rust Web 框架
- **Manganis 0.7.9**: Dioxus 的资源管理和优化系统
- **SCSS**: 强大的 CSS 预处理器

## 🚀 运行示例

### 1. 构建和运行

```bash
cd examples/with-scss-asset
cargo run --release
```

### 2. 开发模式

```bash
cd examples/with-scss-asset
cargo watch -x "run --release"
```

## 💡 核心代码解析

### SCSS 资源声明

```rust
use manganis::*;

// 使用 asset! 宏声明 SCSS 文件
// Dioxus 会在构建时自动编译这个文件
static SCSS_STYLES: Asset = asset!("/assets/theme-chalk.scss");
```

### 在组件中使用

```rust
#[component]
fn App() -> Element {
    rsx! {
        // Manganis 会自动将编译后的 CSS 注入到页面中
        // 不需要手动引入 CSS 文件
        
        div {
            CustomCard {
                title: "SCSS 功能展示",
                children: rsx! {
                    // 使用 theme-chalk 样式的组件
                }
            }
        }
    }
}
```

## 🎨 SCSS 功能展示

### 1. 变量系统

```scss
// 定义颜色变量
$primary-color: #409EFF;
$success-color: #67C23A;
$danger-color: #F56C6C;

// 在样式中使用变量
.el-button--primary {
    background-color: $primary-color;
    border-color: $primary-color;
}
```

### 2. Mixin 混合

```scss
// 定义按钮样式混合
@mixin button-variant($color, $background, $border) {
    color: $color;
    background-color: $background;
    border-color: $border;
    
    &:hover {
        background: darken($background, 5%);
    }
}

// 使用混合
.el-button--primary {
    @include button-variant(#fff, $primary-color, $primary-color);
}
```

### 3. 响应式设计

```scss
// 响应式网格系统
[class*="el-col-"] {
    float: left;
    box-sizing: border-box;
}

// 不同断点的列宽度
@media only screen and (max-width: 768px) {
    .el-col-md-12 {
        width: 50%;
    }
}

@media only screen and (min-width: 992px) {
    @for $i from 1 through 24 {
        .el-col-lg-#{$i} {
            width: calc($i / 24) * 100%;
        }
    }
}
```

## 🔄 与原始构建流程的对比

### 传统方式 (本项目之前的做法)

```bash
# 需要外部工具链
gem install sass
# 或
npm install -g sass

# 手动编译
sass scss/index.scss dist/theme-chalk.css --style=compressed

# 在应用中手动引入
<link rel="stylesheet" href="/dist/theme-chalk.css">
```

### 新的 Dioxus 0.7 方式

```rust
// 只需要声明 SCSS 资源
static STYLES: Asset = asset!("/assets/theme-chalk.scss");

// Dioxus 自动处理其余所有事情：
// 1. 编译 SCSS 到 CSS
// 2. 优化 CSS 输出
// 3. 自动注入到页面
// 4. 开发时热重载
```

## 🚀 性能优势

1. **构建时编译**: SCSS 在应用构建时编译，无需运行时开销
2. **自动优化**: Manganis 会自动优化编译后的 CSS
3. **资源捆绑**: CSS 文件会被优化并捆绑到应用中
4. **缓存友好**: 生成的 CSS 文件会被正确缓存

## 🔗 相关资源

- [Dioxus 官方文档](https://dioxuslabs.com/learn/0.7/)
- [Manganis 资产管理系统](https://github.com/DioxusLabs/dioxus/tree/main/packages/manganis)
- [Sass 官方文档](https://sass-lang.com/documentation/)

## 📝 注意事项

1. **文件路径**: 使用 `asset!()` 宏时，文件路径是相对于包根目录的
2. **开发环境**: 在开发环境中，修改 SCSS 文件会自动触发重新编译
3. **生产构建**: 发布版本时，CSS 会被高度优化和压缩
4. **错误处理**: SCSS 编译错误会在构建时显示，便于调试

这个示例展示了 Dioxus 0.7 现代化的前端开发体验，让 Rust 开发者能够充分利用 SCSS 的强大功能，同时保持简洁的工作流程。