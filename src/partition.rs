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
    pub fn parse(partition_str: &str) -> Result<Self> {
        if partition_str.trim().is_empty() {
            return Err(anyhow!("Partition string cannot be empty"));
        }

        let parts: Vec<&str> = partition_str.split(':').collect();
        let file_path = parts[0].to_string();

        if file_path.trim().is_empty() {
            return Err(anyhow!("File path cannot be empty"));
        }

        if parts.len() == 1 {
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
                for (idx, line) in lines.iter().enumerate().take(end).skip(start - 1) {
                    let i = idx;
                    let line = *line;
                    let line_content = match (self.start_col, self.end_col) {
                        (Some(start_col), Some(end_col)) => {
                            if i == start - 1 && i == end - 1 {
                                let chars: Vec<char> = line.chars().collect();
                                if start_col > chars.len() || end_col > chars.len() {
                                    return Err(anyhow!("Column numbers exceed line length"));
                                }
                                chars[(start_col - 1)..end_col].iter().collect()
                            } else if i == start - 1 {
                                let chars: Vec<char> = line.chars().collect();
                                if start_col > chars.len() {
                                    return Err(anyhow!("Start column exceeds line length"));
                                }
                                chars[(start_col - 1)..].iter().collect()
                            } else if i == end - 1 {
                                let chars: Vec<char> = line.chars().collect();
                                if end_col > chars.len() {
                                    return Err(anyhow!("End column exceeds line length"));
                                }
                                chars[..end_col].iter().collect()
                            } else {
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
            _ => Ok(content),
        }
    }

    #[allow(dead_code)]
    #[allow(clippy::inherent_to_string)]
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
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_file_only() {
        let partition = Partition::parse("src/main.rs").unwrap();
        assert_eq!(partition.file_path, "src/main.rs");
        assert_eq!(partition.start_line, None);
        assert_eq!(partition.end_line, None);
        assert_eq!(partition.start_col, None);
        assert_eq!(partition.end_col, None);
    }

    #[test]
    fn test_parse_with_line_range() {
        let partition = Partition::parse("src/main.rs:10-20").unwrap();
        assert_eq!(partition.file_path, "src/main.rs");
        assert_eq!(partition.start_line, Some(10));
        assert_eq!(partition.end_line, Some(20));
        assert_eq!(partition.start_col, None);
        assert_eq!(partition.end_col, None);
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

    #[test]
    fn test_parse_single_line() {
        let partition = Partition::parse("README.md:42").unwrap();
        assert_eq!(partition.file_path, "README.md");
        assert_eq!(partition.start_line, Some(42));
        assert_eq!(partition.end_line, Some(42));
    }

    #[test]
    fn test_parse_single_column() {
        let partition = Partition::parse("file.txt:10@5").unwrap();
        assert_eq!(partition.file_path, "file.txt");
        assert_eq!(partition.start_line, Some(10));
        assert_eq!(partition.end_line, Some(10));
        assert_eq!(partition.start_col, Some(5));
        assert_eq!(partition.end_col, Some(5));
    }

    #[test]
    fn test_parse_with_empty_ranges() {
        let partition = Partition::parse("file.txt:@").unwrap();
        assert_eq!(partition.file_path, "file.txt");
        assert_eq!(partition.start_line, None);
        assert_eq!(partition.end_line, None);
        assert_eq!(partition.start_col, None);
        assert_eq!(partition.end_col, None);
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = Partition::parse("");
        assert!(result.is_err());

        assert!(Partition::parse("file.txt:abc").is_err());
        assert!(Partition::parse("file.txt:10@abc").is_err());

        assert!(Partition::parse("file.txt:10-5").is_ok());
    }

    #[test]
    fn test_extract_content_entire_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3").unwrap();

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: None,
            end_line: None,
            start_col: None,
            end_col: None,
        };

        let content = partition.extract_content().unwrap();
        assert_eq!(content, "line1\nline2\nline3");
    }

    #[test]
    fn test_extract_content_line_range() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3\nline4").unwrap();

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: Some(2),
            end_line: Some(3),
            start_col: None,
            end_col: None,
        };

        let content = partition.extract_content().unwrap();
        assert_eq!(content, "line2\nline3");
    }

    #[test]
    fn test_extract_content_single_line() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3").unwrap();

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: Some(2),
            end_line: Some(2),
            start_col: None,
            end_col: None,
        };

        let content = partition.extract_content().unwrap();
        assert_eq!(content, "line2");
    }

    #[test]
    fn test_extract_content_with_columns() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello world\nrust programming").unwrap();

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: Some(1),
            end_line: Some(1),
            start_col: Some(7),
            end_col: Some(11),
        };

        let content = partition.extract_content().unwrap();
        assert_eq!(content, "world");
    }

    #[test]
    fn test_extract_content_multiline_with_columns() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello world\nrust programming\ngreat language").unwrap();

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: Some(1),
            end_line: Some(2),
            start_col: Some(7),
            end_col: Some(4),
        };

        let content = partition.extract_content().unwrap();
        assert_eq!(content, "world\nrust");
    }

    #[test]
    fn test_extract_content_file_not_found() {
        let partition = Partition {
            file_path: "nonexistent.txt".to_string(),
            start_line: None,
            end_line: None,
            start_col: None,
            end_col: None,
        };

        assert!(partition.extract_content().is_err());
    }

    #[test]
    fn test_extract_content_invalid_line_numbers() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2").unwrap();

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: Some(0),
            end_line: Some(1),
            start_col: None,
            end_col: None,
        };
        assert!(partition.extract_content().is_err());

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: Some(1),
            end_line: Some(5),
            start_col: None,
            end_col: None,
        };
        assert!(partition.extract_content().is_err());

        let partition = Partition {
            file_path: file_path.to_string_lossy().to_string(),
            start_line: Some(2),
            end_line: Some(1),
            start_col: None,
            end_col: None,
        };
        assert!(partition.extract_content().is_err());
    }

    #[test]
    fn test_to_string() {
        let partition = Partition {
            file_path: "src/main.rs".to_string(),
            start_line: Some(10),
            end_line: Some(20),
            start_col: Some(5),
            end_col: Some(15),
        };
        assert_eq!(partition.to_string(), "src/main.rs:10-20@5-15");

        let partition = Partition {
            file_path: "README.md".to_string(),
            start_line: Some(5),
            end_line: Some(5),
            start_col: None,
            end_col: None,
        };
        assert_eq!(partition.to_string(), "README.md:5");

        let partition = Partition {
            file_path: "file.txt".to_string(),
            start_line: None,
            end_line: None,
            start_col: None,
            end_col: None,
        };
        assert_eq!(partition.to_string(), "file.txt");
    }
}
