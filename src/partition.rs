use anyhow::{anyhow, Result};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Partition {
    pub file_path: String,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub start_col: Option<usize>,
    pub end_col: Option<usize>,
}

impl Partition {
    /// Parse a partition string in the format:
    /// <relative_path>:<start_line>-<end_line>@<start_col>-<end_col>
    /// Line or column ranges can be optional
    pub fn parse(partition_str: &str) -> Result<Self> {
        let parts: Vec<&str> = partition_str.split(':').collect();
        if parts.len() < 1 {
            return Err(anyhow!("Invalid partition format"));
        }

        let file_path = parts[0].to_string();
        
        if parts.len() == 1 {
            // Just a file path, no ranges
            return Ok(Partition {
                file_path,
                start_line: None,
                end_line: None,
                start_col: None,
                end_col: None,
            });
        }

        let range_part = parts[1];
        let (line_range, col_range) = if range_part.contains('@') {
            let range_parts: Vec<&str> = range_part.split('@').collect();
            (range_parts[0], Some(range_parts[1]))
        } else {
            (range_part, None)
        };

        let (start_line, end_line) = if line_range.is_empty() {
            (None, None)
        } else {
            let line_parts: Vec<&str> = line_range.split('-').collect();
            match line_parts.len() {
                1 => {
                    let line = line_parts[0].parse::<usize>()?;
                    (Some(line), Some(line))
                }
                2 => {
                    let start = line_parts[0].parse::<usize>()?;
                    let end = line_parts[1].parse::<usize>()?;
                    (Some(start), Some(end))
                }
                _ => return Err(anyhow!("Invalid line range format")),
            }
        };

        let (start_col, end_col) = if let Some(col_range) = col_range {
            if col_range.is_empty() {
                (None, None)
            } else {
                let col_parts: Vec<&str> = col_range.split('-').collect();
                match col_parts.len() {
                    1 => {
                        let col = col_parts[0].parse::<usize>()?;
                        (Some(col), Some(col))
                    }
                    2 => {
                        let start = col_parts[0].parse::<usize>()?;
                        let end = col_parts[1].parse::<usize>()?;
                        (Some(start), Some(end))
                    }
                    _ => return Err(anyhow!("Invalid column range format")),
                }
            }
        } else {
            (None, None)
        };

        Ok(Partition {
            file_path,
            start_line,
            end_line,
            start_col,
            end_col,
        })
    }

    /// Extract content from the file based on the partition specification
    pub fn extract_content(&self) -> Result<String> {
        let file_path = Path::new(&self.file_path);
        if !file_path.exists() {
            return Err(anyhow!("File not found: {}", self.file_path));
        }

        let content = std::fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        match (self.start_line, self.end_line) {
            (Some(start), Some(end)) => {
                if start == 0 || end == 0 {
                    return Err(anyhow!("Line numbers must be 1-indexed"));
                }
                if start > lines.len() || end > lines.len() {
                    return Err(anyhow!("Line numbers exceed file length"));
                }
                if start > end {
                    return Err(anyhow!("Start line must be <= end line"));
                }

                let mut result = String::new();
                for i in (start - 1)..end {
                    let line = lines[i];
                    let line_content = match (self.start_col, self.end_col) {
                        (Some(start_col), Some(end_col)) => {
                            if i == start - 1 && i == end - 1 {
                                // Single line with column range
                                let chars: Vec<char> = line.chars().collect();
                                if start_col > chars.len() || end_col > chars.len() {
                                    return Err(anyhow!("Column numbers exceed line length"));
                                }
                                chars[(start_col - 1)..end_col].iter().collect()
                            } else if i == start - 1 {
                                // First line with start column
                                let chars: Vec<char> = line.chars().collect();
                                if start_col > chars.len() {
                                    return Err(anyhow!("Start column exceeds line length"));
                                }
                                chars[(start_col - 1)..].iter().collect()
                            } else if i == end - 1 {
                                // Last line with end column
                                let chars: Vec<char> = line.chars().collect();
                                if end_col > chars.len() {
                                    return Err(anyhow!("End column exceeds line length"));
                                }
                                chars[..end_col].iter().collect()
                            } else {
                                // Middle lines, full content
                                line.to_string()
                            }
                        }
                        _ => line.to_string(),
                    };
                    
                    if i > start - 1 {
                        result.push('\n');
                    }
                    result.push_str(&line_content);
                }
                Ok(result)
            }
            _ => {
                // No line range specified, return entire file
                Ok(content)
            }
        }
    }

    /// Convert the partition back to string format
    pub fn to_string(&self) -> String {
        let mut result = self.file_path.clone();
        
        if let (Some(start_line), Some(end_line)) = (self.start_line, self.end_line) {
            if start_line == end_line {
                result.push_str(&format!(":{}", start_line));
            } else {
                result.push_str(&format!(":{}-{}", start_line, end_line));
            }
        }

        if let (Some(start_col), Some(end_col)) = (self.start_col, self.end_col) {
            if start_col == end_col {
                result.push_str(&format!("@{}", start_col));
            } else {
                result.push_str(&format!("@{}-{}", start_col, end_col));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file_only() {
        let partition = Partition::parse("src/main.rs").unwrap();
        assert_eq!(partition.file_path, "src/main.rs");
        assert_eq!(partition.start_line, None);
        assert_eq!(partition.end_line, None);
    }

    #[test]
    fn test_parse_with_line_range() {
        let partition = Partition::parse("src/main.rs:10-20").unwrap();
        assert_eq!(partition.file_path, "src/main.rs");
        assert_eq!(partition.start_line, Some(10));
        assert_eq!(partition.end_line, Some(20));
    }

    #[test]
    fn test_parse_with_line_and_column_range() {
        let partition = Partition::parse("src/main.rs:10-20@5-15").unwrap();
        assert_eq!(partition.file_path, "src/main.rs");
        assert_eq!(partition.start_line, Some(10));
        assert_eq!(partition.end_line, Some(20));
        assert_eq!(partition.start_col, Some(5));
        assert_eq!(partition.end_col, Some(15));
    }
} 