//! menu-item-group component module
//! Generated Element Plus component

/// MenuItemGroup component classes
pub mod classes {
    /// Base menu-item-group class
    pub const BASE: &str = "el-menu-item-group";
    
    /// menu-item-group size variants
    pub const LARGE: &str = "el-menu-item-group--large";
    pub const SMALL: &str = "el-menu-item-group--small";
    
    /// menu-item-group type variants
    pub const PRIMARY: &str = "el-menu-item-group--primary";
    pub const SUCCESS: &str = "el-menu-item-group--success";
    pub const WARNING: &str = "el-menu-item-group--warning";
    pub const DANGER: &str = "el-menu-item-group--danger";
    pub const INFO: &str = "el-menu-item-group--info";
    
    /// menu-item-group states
    pub const ACTIVE: &str = "is-active";
    pub const DISABLED: &str = "is-disabled";
    pub const FOCUS: &str = "is-focus";
}

/// Basic menu-item-group component structure
#[derive(Debug, Clone)]
pub struct MenuItemGroup {
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub active: bool,
    pub disabled: bool,
}

impl Default for MenuItemGroup {
    fn default() -> Self {
        Self {
            id: None,
            class: None,
            style: None,
            active: false,
            disabled: false,
        }
    }
}

impl MenuItemGroup {
    /// Create a new menu-item-group component
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the component ID
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
    
    /// Set the component class
    pub fn class(mut self, class_name: &str) -> Self {
        self.class = Some(class_name.to_string());
        self
    }
    
    /// Set the component style
    pub fn style(mut self, style_value: &str) -> Self {
        self.style = Some(style_value.to_string());
        self
    }
    
    /// Set active state
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    
    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    /// Generate CSS class names for the component
    pub fn generate_class_names(&self) -> Vec<String> {
        let mut class_names = Vec::new();
        
        // Add base class
        class_names.push(classes::BASE.to_string());
        
        // Add state classes
        if self.active {
            class_names.push(classes::ACTIVE.to_string());
        }
        
        if self.disabled {
            class_names.push(classes::DISABLED.to_string());
        }
        
        // Add custom class if provided
        if let Some(ref custom_class) = self.class {
            class_names.push(custom_class.to_string());
        }
        
        class_names
    }
    
    /// Get HTML representation for testing
    pub fn get_html_info(&self) -> ComponentInfo {
        ComponentInfo {
            component_type: "menu-item-group".to_string(),
            class_names: self.generate_class_names(),
            id: self.id.clone(),
            style: self.style.clone(),
        }
    }
}

/// Component information for testing
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub component_type: String,
    pub class_names: Vec<String>,
    pub id: Option<String>,
    pub style: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_menu_item_group_creation() {
        let component = MenuItemGroup::new()
            .id("test-menu-item-group")
            .class("custom-menu-item-group-class");
            
        assert_eq!(component.id.as_ref().unwrap(), "test-menu-item-group");
        assert_eq!(component.class.as_ref().unwrap(), "custom-menu-item-group-class");
        assert_eq!(component.active, false);
        assert_eq!(component.disabled, false);
    }
    
    #[test]
    fn test_menu_item_group_class_generation() {
        let component = MenuItemGroup::new()
            .active(true)
            .disabled(false)
            .class("extra-class");
            
        let class_names = component.generate_class_names();
        
        assert!(class_names.contains(&classes::BASE.to_string()));
        assert!(class_names.contains(&classes::ACTIVE.to_string()));
        assert!(!class_names.contains(&classes::DISABLED.to_string()));
        assert!(class_names.contains(&"extra-class".to_string()));
    }
    
    #[test]
    fn test_menu_item_group_states() {
        let active_disabled = MenuItemGroup::new()
            .active(true)
            .disabled(true);
            
        let class_names = active_disabled.generate_class_names();
        
        assert!(class_names.contains(&classes::ACTIVE.to_string()));
        assert!(class_names.contains(&classes::DISABLED.to_string()));
    }
}
