//! # CSS Snippets
//!
//! Reusable CSS snippets for Element Plus components.

/// CSS Reset snippet
pub fn css_reset() -> &'static str {
    r#"
/* CSS Reset */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

*:before, *:after {
    box-sizing: inherit;
}
"#
}

/// Basic utility classes
pub fn utility_classes() -> &'static str {
    r#"
/* Basic utilities */
.clearfix::before,
.clearfix::after {
    display: table;
    content: "";
}

.clearfix::after {
    clear: both;
}

.pull-left {
    float: left;
}

.pull-right {
    float: right;
}

.text-center {
    text-align: center;
}

.text-left {
    text-align: left;
}

.text-right {
    text-align: right;
}
"#
}

/// Grid system snippet
pub fn grid_system() -> &'static str {
    r#"
/* Grid System */
.el-row {
    display: flex;
    flex-wrap: wrap;
}

.el-col {
    position: relative;
    max-width: 100%;
    min-height: 1px;
}

/* Column widths */
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
"#
}
