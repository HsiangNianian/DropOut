// utils/mod.rs
pub mod file_utils;
pub mod config_parser;

// 文件操作相关的实用工具函数
pub mod file_utils {
    use std::fs;
    use std::io::{self, Write};

    pub fn read_file_to_string(path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn write_string_to_file(path: &str, content: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())
    }
}

// 配置解析相关的实用工具函数
pub mod config_parser {
    use std::collections::HashMap;

    pub fn parse_config(content: &str) -> HashMap<String, String> {
        let mut config = HashMap::new();
        for line in content.lines() {
            let mut parts = line.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                config.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        config
    }
}