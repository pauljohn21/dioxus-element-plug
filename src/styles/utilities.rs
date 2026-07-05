//! # Utility Classes
//!
//! Utility classes and helper functions for Element Plus styling.

/// Generate responsive grid classes
pub fn generate_grid_classes() -> String {
    let mut grid_css = String::new();
    
    grid_css.push_str(r#"
/* Row */
.el-row {
    display: flex;
    flex-wrap: wrap;
    position: relative;
}

/* Column system */
.el-col-1 { width: 4.1666666667%; }
.el-col-2 { width: 8.3333333333%; }
.el-col-3 { width: 12.5%; }
.el-col-4 { width: 16.6666666667%; }
.el-col-5 { width: 20.8333333333%; }
.el-col-6 { width: 25%; }
.el-col-7 { width: 29.1666666667%; }
.el-col-8 { width: 33.3333333333%; }
.el-col-9 { width: 37.5%; }
.el-col-10 { width: 41.6666666667%; }
.el-col-11 { width: 45.8333333333%; }
.el-col-12 { width: 50%; }
.el-col-13 { width: 54.1666666667%; }
.el-col-14 { width: 58.3333333333%; }
.el-col-15 { width: 62.5%; }
.el-col-16 { width: 66.6666666667%; }
.el-col-17 { width: 70.8333333333%; }
.el-col-18 { width: 75%; }
.el-col-19 { width: 79.1666666667%; }
.el-col-20 { width: 83.3333333333%; }
.el-col-21 { width: 87.5%; }
.el-col-22 { width: 91.6666666667%; }
.el-col-23 { width: 95.8333333333%; }
.el-col-24 { width: 100%; }
"#);
    
    grid_css
}

/// Generate display utility classes
pub fn generate_display_utilities() -> String {
    String::from(r#"
/* Display utilities */
.d-none { display: none; }
.d-block { display: block; }
.d-inline { display: inline; }
.d-inline-block { display: inline-block; }
.d-flex { display: flex; }
.d-inline-flex { display: inline-flex; }
"#)
}