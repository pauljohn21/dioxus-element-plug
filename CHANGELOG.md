# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-07-04

### Added
- **Core Library Structure**
  - Initial project setup with Cargo.toml configuration
  - Build system with Makefile for CSS compilation
  - Rust toolchain configuration

- **Button Component**
  - Full button implementation with theme-chalk styling
  - Support for all variants: primary, success, warning, danger, info
  - Size options: large, medium, small, mini
  - Special button types: OutlineButton, TextButton, LinkButton
  - Features: disabled state, loading state, icons, round/circle styles

- **Input Component**
  - Complete input implementation with validation states
  - Support for all input types: text, password, email, number, etc.
  - Size variants matching theme-chalk design
  - Features: clearable, show password toggle, prefix/suffix icons
  - Specialized inputs: SearchInput, PasswordInput

- **Layout Components**
  - Container system: Container, Header, Aside, Main, Footer
  - Grid system: Row and Col components with responsive options
  - Proper spacing and alignment controls

- **Theme System**
  - Complete theme constants from theme-chalk
  - CSS classes, colors, typography, spacing, and border constants
  - Type-safe theme configuration

- **Build System**
  - SCSS to CSS compilation with sass
  - Build script for version information
  - Makefile with development targets
  - npm setup for CSS dependencies

- **Documentation**
  - Comprehensive README with examples
  - Component usage guides and API documentation
  - Quick start examples
  - Development setup instructions

- **Examples**
  - Basic example application demonstrating component usage
  - Interactive demo with buttons, inputs, and grid layout
  - Ready-to-run example with proper setup

### Changed
- None (initial release)

### Deprecated
- None

### Removed
- None

### Fixed
- None

### Security
- None

## Planned Features

### Next Release (0.2.0)
- [ ] Form components (Checkbox, Radio, Select, Switch)
- [ ] Navigation components (Menu, Tabs, Breadcrumb, Pagination)
- [ ] Feedback components (Alert, Message, Notification, Loading)
- [ ] Data display components (Table, Card, List, Timeline)
- [ ] Advanced components (Modal, Tooltip, Progress, Slider)
- [ ] Icon system integration
- [ ] Dark theme support
- [ ] TypeScript definitions for web components
- [ ] Better error handling and validation
- [ ] Accessibility improvements (ARIA labels, keyboard navigation)

### Future Versions
- [ ] Component storybook/design system documentation
- [ ] Server-side rendering improvements
- [ ] Mobile-first responsive utilities
- [ ] Animation and transition helpers
- [ ] Internationalization support
- [ ] Performance optimizations
- [ ] Testing utilities and examples
