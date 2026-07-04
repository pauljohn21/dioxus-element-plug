# 发布到 crates.io 指南

本指南说明如何将 dioxus-element-plug 发布到 crates.io。

## 📋 发布前准备

### 1. 检查准备状态

```bash
# 测试发布 (dry-run)
cargo publish --dry-run --registry crates-io

# 如果需要忽略未提交的更改
cargo publish --dry-run --registry crates-io --allow-dirty
```

### 2. 版本号更新

决定新版本号的规则：
- **小版本更新** (0.1.x): 向后兼容的错误修复 
- **中等版本更新** (0.x.0): 向后兼容的新功能
- **大版本更新** (x.0.0): 破坏性更改

更新 Cargo.toml:
```toml
[package]
version = "0.1.1"  # 更新为新的版本号
```

### 3. 更新 CHANGELOG

确保 CHANGELOG.md 包含了所有更新的内容：

```markdown
## [0.1.1] - 2025-01-20

### Added
- ✨ 新增 Dioxus 0.7 内置 SCSS 支持
- 📚 添加现代化示例项目
- 🛠️ 优化开发者体验

### Changed
- 🏠 重构项目结构
- 📝 更新文档系统

### Fixed
- 🔧 修复 Makefile 构建问题
- ⚠️ 清理编译警告

## [0.1.0] - 2025-01-01

- 初始发布版本
```

## 🚀 发布流程

### 1. 登录 crates.io

```bash
# 首次登录 (需要 crates.io API token)
cargo login
# 运行后会提示输入 API token
```

### 2. 本地测试

```bash
# 测试编译
cargo check
cargo test

# 检查是否所有需要的功能都能正常工作
cargo check --features manganis

# 生成文档检查
cargo doc --no-deps --features manganis
```

### 3. 正式发布

```bash
# 正式发布到 crates.io
cargo publish --registry crates-io

# 如果还有未提交的更改，可以添加 --allow-dirty
cargo publish --registry crates-io --allow-dirty
```

### 4. 发布后验证

发布完成后：

```bash
# 检查是否成功发布
cargo search dioxus-element-plug

# 在另一个项目测试安装
cd /tmp
touch test_project
cd test_project
echo 'dioxus-element-plug = "0.1.1"' >> Cargo.toml
cargo check
```

## 📦 发布的包内容

根据 `exclude` 配置，以下文件会被包含在发布的包中：

**包含的文件**:
- 📁 `src/` - 所有源代码文件
- 📁 `scss/` - SCSS 样式文件
- 📄 `Cargo.toml` - 包配置
- 📄 `LICENSE` - 许可证
- 📄 `README.md` - 主要文档

**排除的文件**:
- 📁 `examples/` - 示例项目 (留给用户在 GitHub 查看)
- 📁 `dist/` - 编译后的 CSS
- 📁 `.github/` - GitHub 配置
- 📄 `Makefile` - 构建脚本
- 📄 `*_.md` - 大部分文档文件 (留给用户在 GitHub 查看)
- 📄 `CHANGELOG.md` - 变更日志

## 🔄 版本发布策略

### 功能发布流程

1. **开发阶段** - 在 `main` 分支开发新功能
2. **文档更新** - 更新所有相关文档
3. **版本准备** - 更新 Cargo.toml 中的版本号
4. **变更日志** - 更新 CHANGELOG.md
5. **预发布测试** - 使用 `--dry-run` 测试
6. **正式发布** - `cargo publish`
7. **GitHub Release** - 创建对应的 GitHub Release

### 版本号规则

这个项目遵循语义化版本控制：

- `0.1.1` - 小功能更新  
- `0.2.0` - 大功能更新，向后兼容
- `1.0.0` - 稳定版，包含破坏性更改
- `1.1.0` - 稳定版功能添加
- `1.1.1` - 稳定版 bug 修复

## ⚠️ 注意事项

### 包大小限制
crates.io 对包大小有限制 (当前是 10MB，压缩后 1MB)

- 当前包: 145.4KB 压缩后 ✅ 安全
- 监控: `cargo publish --dry-run` 会显示包大小
- 减少大小: 使用 `exclude` 配置排除不必要的文件

### 依赖检查

- **直接依赖**: 确保所有依赖都是必需的
- **可选依赖**: 使用 `[features]` 正确标记
- **版本范围**: 使用合理的版本范围，不要过度约束

### 元数据完整性

确保以下 Cargo.toml 字段都正确设置：
- ✅ `name` - 唯一且在 crates.io 上可用
- ✅ `description` - 清晰描述包的功能  
- ✅ `repository` - 正确的 GitHub 仓库地址
- ✅ `license` - 有效的许可证
- ✅ `authors` - 正确的作者信息
- ✅ `keywords` - 相关搜索关键词
- ✅ `categories` - 合适的分类

## 🚨 常见问题

### Q: 发布失败怎么办？
A: 查看错误消息，常见问题是：
- 网络问题 - 重试
- 版本冲突 - 版本号太小，需要增加
- 依赖问题 - 检查依赖版本和可用性

### Q: 发布后发现错误怎么办？
A: crates.io 不允许删除已发布的版本：
- 发布修复版本 (需要增加版本号)
- 在 README 中指出错误的版本
- 更新 CHANGELOG 记录修复

### Q: 如何撤回版本？
A: 不能删除版本，但可以使用 `cargo yank`：

```bash
# 撤回某个版本 (仍然可以被现有项目使用)
cargo yank --vers 0.1.0

# 取消撤回
cargo yank --vers 0.1.0 --undo
```

## 📞 获得帮助

### crates.io 支持
- 网站: https://crates.io
- GitHub: https://github.com/rust-lang/crates.io
- 问题追踪器: https://github.com/rust-lang/crates.io/issues

### Rust 社区
- Discord: Rust Community
- Reddit: r/rust
- Stack Overflow: 使用标签 `rust` 和 `crates.io`

## 🎉 恭喜发布成功！

发布成功后：

1. 🔗 分享包的链接: `https://crates.io/crates/dioxus-element-plug`
2. 📖 确保文档是最新的: `https://docs.rs/dioxus-element-plug`
3. 🐦 在社交媒体上宣布
4. 📈 监控下载统计数据
5. 💬 收集用户反馈

**祝你发布顺利！** 🚀
