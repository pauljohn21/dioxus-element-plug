//! # CSS Generation
//!
//! Complete CSS generation functions for all Element Plus components.
//! Generated from clean CSS source.

/// Generate all Element Plus styles
pub fn all_styles() -> String {
    String::from(r#"
/* CSS Reset */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

/* Element Plus Base Styles */
.el-button {
    display: inline-block;
    line-height: 1;
    white-space: nowrap;
    cursor: pointer;
    background: #fff;
    border: 1px solid #dcdfe6;
    color: #606266;
    text-align: center;
    box-sizing: border-box;
    outline: none;
    margin: 0;
    transition: .1s;
    font-weight: 500;
    padding: 12px 20px;
    font-size: 14px;
    border-radius: 4px;
}

.el-button:hover,
.el-button:focus {
    color: #409eff;
    border-color: #c6e2ff;
    background-color: #ecf5ff;
}

.el-button--primary {
    color: #fff;
    background-color: #409eff;
    border-color: #409eff;
}

.el-button--primary:hover,
.el-button--primary:focus {
    background: #66b1ff;
    border-color: #66b1ff;
    color: #fff;
}

.el-button--success {
    color: #fff;
    background-color: #67c23a;
    border-color: #67c23a;
}

.el-button--success:hover,
.el-button--success:focus {
    background: #85ce61;
    border-color: #85ce61;
    color: #fff;
}

.el-button--warning {
    color: #fff;
    background-color: #e6a23c;
    border-color: #e6a23c;
}

.el-button--warning:hover,
.el-button--warning:focus {
    background: #ebb563;
    border-color: #ebb563;
    color: #fff;
}

.el-button--danger {
    color: #fff;
    background-color: #f56c6c;
    border-color: #f56c6c;
}

.el-button--danger:hover,
.el-button--danger:focus {
    background: #f78989;
    border-color: #f78989;
    color: #fff;
}

.el-input {
    position: relative;
    font-size: 14px;
    display: inline-block;
    width: 100%;
}

.el-input__wrapper {
    display: inline-flex;
    flex-grow: 1;
    align-items: center;
    justify-content: center;
    padding: 1px 11px;
    background-color: #fff;
    background-image: none;
    border-radius: 4px;
    cursor: text;
    transition: border-color .2s cubic-bezier(.645,.045,.355,1);
    transform: translateZ(0);
    box-shadow: 0 0 0 1px #dcdfe6 inset;
}

.el-input__inner {
    width: 100%;
    flex-grow: 1;
    -webkit-appearance: none;
    color: #606266;
    font-size: inherit;
    height: 32px;
    line-height: 32px;
    padding: 0;
    outline: none;
    border: none;
    background: none;
    box-sizing: border-box;
}

.el-input__inner::placeholder {
    color: #c0c4cc;
}

.el-input__wrapper:hover {
    box-shadow: 0 0 0 1px #c0c4cc inset;
}

.el-input__wrapper.is-focus {
    box-shadow: 0 0 0 1px #409eff inset!important;
}

.el-form {
    margin: 0;
    padding: 0;
}

.el-form-item {
    margin-bottom: 22px;
}

.el-form-item__label {
    float: left;
    font-size: 14px;
    color: #606266;
    line-height: 40px;
    padding: 0 12px 0 0;
    box-sizing: border-box;
}

.el-form-item__content {
    line-height: 40px;
    position: relative;
    font-size: 14px;
}

.el-alert {
    width: 100%;
    padding: 8px 16px;
    margin: 0;
    box-sizing: border-box;
    border-radius: 4px;
    position: relative;
    background-color: #fff;
    overflow: hidden;
    opacity: 1;
    display: flex;
    align-items: center;
    transition: opacity .2s;
}

.el-alert--success {
    background-color: #f0f9ff;
    color: #67c23a;
}

.el-alert--info {
    background-color: #f4f4f5;
    color: #909399;
}

.el-alert--warning {
    background-color: #fdf6ec;
    color: #e6a23c;
}

.el-alert--error {
    background-color: #fef0f0;
    color: #f56c6c;
}

.el-card {
    border-radius: 4px;
    border: 1px solid #ebeef5;
    background-color: #fff;
    overflow: hidden;
    color: #303133;
    transition: .3s;
}

.el-card__header {
    padding: 18px 20px;
    border-bottom: 1px solid #ebeef5;
    box-sizing: border-box;
}

.el-card__body {
    padding: 20px;
}

.el-card.is-hover-shadow:hover,
.el-card.is-hover-shadow:focus {
    box-shadow: 0 2px 12px 0 rgba(0,0,0,.1);
}

.el-table {
    position: relative;
    overflow: hidden;
    box-sizing: border-box;
    height: fit-content;
    width: 100%;
    max-width: 100%;
    background-color: #fff;
    font-size: 14px;
    color: #606266;
}

.el-table th {
    white-space: nowrap;
    overflow: hidden;
    user-select: none;
    text-align: left;
    background-color: #fafafa;
}

.el-table th.is-leaf {
    border-bottom: 1px solid #ebeef5;
}

.el-table td {
    padding: 12px 0;
    min-width: 0;
    box-sizing: border-box;
    text-overflow: ellipsis;
    vertical-align: middle;
    position: relative;
}

.el-row {
    position: relative;
    box-sizing: border-box;
    display: flex;
    flex-wrap: wrap;
    margin-right: 0;
    margin-left: 0;
}

.el-col {
    position: relative;
    max-width: 100%;
    display: flex;
}

/* Grid system */
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
"#)
}
