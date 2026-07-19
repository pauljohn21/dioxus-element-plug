# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-07-19

### Added

- **Element Plus Icons Integration** - Optional `icons` feature with 137+ SVG icons
  - Button: Loading icon for loading state
  - Popconfirm: QuestionFilled icon
  - Progress: CircleCheck, CircleClose, Warning icons for status indication
  - Card: ArrowDown, ArrowRight icons for accordion
  - Cascader: ArrowRight icon for navigation
  - Autocomplete: CircleClose icon for clear button
  - Transfer: ArrowLeft, ArrowRight icons for transfer buttons
- **Dark Mode Support** - Built-in `Theme::dark()` and `Theme::light()` presets
- **Unified Style System** - `Theme` with 50 customizable fields
- **Documentation** - Added `.cursorrules`, `CLAUDE.md`, `CODING_STANDARDS.md`

### Changed

- All icon-using components now support conditional compilation with `#[cfg(feature = "icons")]`
- CSS class fallbacks maintained for non-icon builds
- Component showcase updated to demonstrate icon usage

### Fixed

- Transfer component selection and move logic
- Various clippy warnings across component files

## [0.2.0] - 2026-07-15

### Added

- Initial release with 107+ components
- Pure Rust CSS generation via `CompleteStyleManager`
- Controlled component pattern throughout
- Element Plus theme-chalk CSS class naming
- Form components: Button, Input, Select, Switch, Checkbox, Radio, etc.
- Data display: Table, Tree, Card, Tag, Progress, Badge
- Navigation: Menu, Tabs, Breadcrumb, Pagination, Steps
- Feedback: Dialog, Drawer, Alert, Message, Notification, Tooltip
- Layout: Container, Row/Col, Space

### Technical

- Dioxus 0.7 framework support
- Zero runtime CSS overhead
- No CDN or external CSS dependencies
