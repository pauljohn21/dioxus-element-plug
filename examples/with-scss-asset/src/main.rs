use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;
use manganis::asset;

// 使用 manganis 的 asset 宏直接引入 SCSS 文件
// Dioxus 会在构建时自动编译 SCSS 并将其包含在应用中
#[allow(dead_code)]
static SCSS_STYLES: Asset = asset!("/assets/theme-chalk.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        // 在 Dioxus 0.7 中，manganis 会自动处理 SCSS 文件的引入
        // 编译后的 CSS 会被自动注入到页面中
        
        div {
            style: "padding: 20px;",
            h1 { "Dioxus Theme Chalk Example with Built-in SCSS" }
            p { style: "color: #666; margin-bottom: 20px;", "This example demonstrates using SCSS files directly with Dioxus 0.7's built-in support via manganis asset macro." }
            
            // 创建一个自定义卡片组件来演示 SCSS 功能
            CustomCard {
                title: "SCSS 功能展示",
                children: rsx! {
                    div {
                        Button {
                            variant: ButtonVariant::Primary,
                            on_click: move |_| counter += 1,
                            "Count: {counter}"
                        }
                        
                        Button {
                            variant: ButtonVariant::Success,
                            "Success Button"
                        }
                        
                        OutlineButton {
                            variant: ButtonVariant::Danger,
                            "Outline Button"
                        }
                    }
                }
            }
            
            // 展示不同的组件
            h3 { "表单组件" }
            div {
                style: "display: flex; flex-direction: column; gap: 15px; margin: 20px 0;",
                
                Input {
                    placeholder: "输入您的姓名...",
                    size: Some(InputSize::Large),
                }
                
                PasswordInput {
                    placeholder: "输入密码...",
                    clearable: true,
                }
            }
            
            // 响应式布局示例
            h3 { "响应式布局" }
            ResponsiveGrid {}
        }
    }
}

// 自定义卡片组件，使用 theme-chalk 的样式
#[component]
fn CustomCard(
    title: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            // 使用 Element UI 的卡片样式类
            class: "el-card",
            style: "margin: 20px 0;",
            
            // 卡片头部
            div {
                class: "el-card__header",
                style: "padding: 18px 20px; border-bottom: 1px solid #ebeef5; box-sizing: border-box;",
                h3 { style: "margin: 0; color: #303133;", "{title}" }
            }
            
            // 卡片内容
            div {
                class: "el-card__body",
                style: "padding: 20px;",
                
                {children}
            }
        }
    }
}

// 响应式网格组件
#[component]
fn ResponsiveGrid() -> Element {
    rsx! {
        div {
            class: "el-row",
            style: "margin: 20px 0;",
            
            // 移动端：24列，平板：12列，桌面：6列
            div {
                class: "el-col-24 el-col-md-12 el-col-lg-6",
                div {
                    style: "padding: 24px; background: #f0f9ff; border: 1px solid #e1f5fe; border-radius: 4px; margin: 5px;",
                    h4 { "响应式列 1" }
                    p { style: "color: #666; font-size: 14px;", "在移动设备上占满宽度" }
                }
            }
            
            div {
                class: "el-col-24 el-col-md-12 el-col-lg-6",
                div {
                    style: "padding: 24px; background: #f6ffed; border: 1px solid #f0f0f0; border-radius: 4px; margin: 5px;",
                    h4 { "响应式列 2" }
                    p { style: "color: #666; font-size: 14px;", "在平板设备上占一半宽度" }
                }
            }
            
            div {
                class: "el-col-24 el-col-md-12 el-col-lg-6",
                div {
                    style: "padding: 24px; background: #fff7e6; border: 1px solid #faecd8; border-radius: 4px; margin: 5px;",
                    h4 { "响应式列 3" }
                    p { style: "color: #666; font-size: 14px;", "在桌面设备上占四分之一宽度" }
                }
            }
            
            div {
                class: "el-col-24 el-col-md-12 el-col-lg-6",
                div {
                    style: "padding: 24px; background: #fef7f0; border: 1px solid #fde2e2; border-radius: 4px; margin: 5px;",
                    h4 { "响应式列 4" }
                    p { style: "color: #666; font-size: 14px;", "完美的响应式体验" }
                }
            }
        }
    }
}
