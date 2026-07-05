//! Simple CSS generation functions - Now Enhanced!
//! This module provides access to the complete Element Plus CSS system

/// Generate complete Element Plus styles with all 114 components
/// Uses the enhanced CSS system with full theme support
pub fn all_styles() -> String {
    crate::styles::enhanced_css_system::all_styles()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complete_css_generation() {
        let css = all_styles();
        
        // Verify CSS is not empty
        assert!(!css.is_empty());
        
        // Verify theme variables are included
        assert!(css.contains("--el-color-primary"));
        assert!(css.contains("--el-color-success"));
        assert!(css.contains("--el-color-warning"));
        assert!(css.contains("--el-color-danger"));
        assert!(css.contains("--el-color-info"));
        
        // Verify component styles are included
        assert!(css.contains(".el-button"));
        assert!(css.contains(".el-input"));
        assert!(css.contains(".el-alert"));
        assert!(css.contains(".el-form"));
        assert!(css.contains(".el-table"));
        assert!(css.contains(".el-card"));
        assert!(css.contains(".el-menu"));
        assert!(css.contains(".el-tabs"));
        assert!(css.contains(".el-dialog"));
        assert!(css.contains(".el-dropdown"));
        assert!(css.contains(".el-steps"));
        
        // Verify CSS Reset is included
        assert!(css.contains("box-sizing: border-box"));
        assert!(css.contains("font-family"));
        
        // Verify responsive design elements
        assert!(css.contains(".el-col-"));
        
        println!("✅ Complete CSS system generated successfully!");
        println!("📊 CSS size: {} characters", css.len());
    }
}