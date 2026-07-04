# SCSS Structure Documentation

This document describes the organized structure of the Theme Chalk SCSS files.

## Directory Structure

```
scss/
├── index.scss                 # Main entry point that imports all components
├── common/                    # Base styles and utilities
│   ├── var.scss              # Variables (colors, typography, spacing)
│   ├── reset.scss            # CSS reset/normalize
│   ├── base.scss             # Base element styles
│   ├── display.scss          # Display utilities
│   ├── transition.scss       # Transition utilities
│   └── fonts/                # Font files and icon fonts
│       ├── element-icons.ttf
│       └── element-icons.woff
├── mixins/                    # SCSS mixins and functions
│   ├── function.scss         # Helper functions
│   ├── mixins.scss           # Common mixins
│   ├── utils.scss            # Utility mixins
│   ├── _button.scss          # Button-specific mixins
│   └── config.scss           # Configuration mixins
├── layout/                    # Layout components
│   ├── container.scss        # Main container
│   ├── header.scss          # Page header
│   ├── aside.scss           # Sidebar
│   ├── main.scss            # Main content area
│   ├── footer.scss          # Page footer
│   ├── row.scss             # Grid row
│   └── col.scss             # Grid column
├── form/                      # Form components
│   ├── input.scss            # Text inputs
│   ├── input-number.scss     # Number inputs
│   ├── form.scss             # Form container
│   └── form-item.scss        # Form item wrapper
├── components/                # UI components (alphabetical)
│   ├── autocomplete.scss
│   ├── backtop.scss
│   ├── button.scss           # (Button styles)
│   ├── button-group.scss
│   ├── calendar.scss
│   ├── carousel.scss
│   ├── carousel-item.scss
│   ├── cascader.scss
│   ├── cascader-panel.scss
│   ├── checkbox.scss
│   ├── checkbox-button.scss
│   ├── checkbox-group.scss
│   ├── collapse.scss
│   ├── collapse-item.scss
│   ├── color-picker.scss
│   ├── date-picker.scss
│   ├── descriptions.scss
│   ├── descriptions-item.scss
│   ├── dialog.scss
│   ├── divider.scss
│   ├── drawer.scss
│   ├── icon.scss
│   ├── infinite-scroll.scss
│   ├── infiniteScroll.scss   # (Legacy naming)
│   ├── link.scss
│   ├── option.scss
│   ├── option-group.scss
│   ├── page-header.scss
│   ├── pagination.scss
│   ├── popconfirm.scss
│   ├── popover.scss
│   ├── popper.scss
│   ├── progress.scss
│   ├── radio.scss
│   ├── radio-button.scss
│   ├── radio-group.scss
│   ├── rate.scss
│   ├── scrollbar.scss
│   ├── select.scss
│   ├── select-dropdown.scss
│   ├── skeleton.scss
│   ├── skeleton-item.scss
│   ├── slider.scss
│   ├── step.scss
│   ├── steps.scss
│   ├── switch.scss
│   ├── tag.scss
│   ├── time-picker.scss
│   ├── time-select.scss
│   ├── timeline.scss
│   ├── timeline-item.scss
│   ├── tooltip.scss
│   ├── transfer.scss
│   ├── tree.scss
│   └── upload.scss
├── data/                      # Data display components
│   ├── avatar.scss
│   ├── badge.scss
│   ├── card.scss
│   ├── empty.scss
│   ├── image.scss
│   ├── table.scss
│   └── table-column.scss
├── feedback/                  # Feedback and messaging components
│   ├── alert.scss
│   ├── loading.scss
│   ├── message.scss
│   ├── message-box.scss
│   ├── notification.scss
│   ├── result.scss
│   └── spinner.scss
├── navigation/                # Navigation components
│   ├── breadcrumb.scss
│   ├── breadcrumb-item.scss
│   ├── dropdown.scss
│   ├── dropdown-menu.scss
│   ├── dropdown-item.scss
│   ├── menu.scss
│   ├── menu-item.scss
│   ├── menu-item-group.scss
│   ├── submenu.scss
│   ├── tab-pane.scss
│   └── tabs.scss
└── date-picker/              # Complex date picker components (legacy location)
    ├── picker.scss
    ├── picker-panel.scss
    ├── date-picker.scss
    ├── date-range-picker.scss
    ├── date-table.scss
    ├── month-table.scss
    ├── year-table.scss
    ├── time-picker.scss
    ├── time-range-picker.scss
    └── time-spinner.scss
```

## Import Order in index.scss

The `index.scss` file imports components in a specific order to ensure proper CSS cascade:

1. **Core Variables** - Base theme variables and functions
2. **Mixins** - Utility functions and mixins
3. **Base Styles** - Reset, base element styles, and utilities
4. **Layout** - Grid system and page layout components
5. **Form Components** - Input fields and form controls
6. **Button Components** - Button styles and variants
7. **Data Display** - Tables, cards, and data visualization
8. **Feedback** - Messages, alerts, and loading indicators
9. **Navigation** - Menus, tabs, and breadcrumbs
10. **UI Components** - All other components alphabetically
11. **Date Picker** - Complex date/time picker components

## Naming Convention

- All files use kebab-case (e.g., `button-group.scss`)
- Component names match Element UI's naming convention
- Legacy files that don't follow the convention are marked (e.g., `infiniteScroll.scss`)

## Development Guidelines

### Adding New Components

1. **Create the SCSS file** in the appropriate category directory
2. **Follow existing patterns** - Use the same variables and mixins
3. **Add to index.scss** - Import the new file in the appropriate section
4. **Test the build** - Run `make build-css` to ensure no compilation errors

### Modifying Existing Components

1. **Locate the component** in the organized structure
2. **Make changes** following Element UI design patterns
3. **Test thoroughly** - Ensure changes don't break other components
4. **Update documentation** if the change affects the overall structure

### Building CSS

```bash
# Build CSS once
make build-css

# Watch for changes during development
make watch-css

# Clean and rebuild
make clean && make build-css
```

## Performance Considerations

- **Modular structure** allows for component-level CSS extraction
- **Alphabetical ordering** in components section makes it easy to find files
- **Logical grouping** reduces cognitive overhead when working with the codebase
- **Variable centralization** in `common/var.scss` makes theme customization easy

## Customization

To customize the theme:

1. **Override variables** in `common/var.scss`
2. **Add custom mixins** in `mixins/` directory
3. **Create custom components** in appropriate category directories
4. **Import custom styles** after the main `index.scss` import

## Migration Notes

This structure was created from the original flat `src/` directory to improve maintainability. The migration:

- **Preserves all original functionality** - No breaking changes
- **Improves organization** - Logical grouping by component type
- **Enhances maintainability** - Easier to find and modify components
- **Supports scalable growth** - Easy to add new components in the future

For questions about the structure or to suggest improvements, please refer to the project's contribution guidelines.
