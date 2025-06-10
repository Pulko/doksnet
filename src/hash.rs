pub fn hash_content(content: &str) -> String {
    let hash = blake3::hash(content.as_bytes());
    hash.to_hex().to_string()
}

pub fn verify_hash(content: &str, expected_hash: &str) -> bool {
    let actual_hash = hash_content(content);
    actual_hash == expected_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_content() {
        let content = "Hello, world!";
        let hash = hash_content(content);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_verify_hash() {
        let content = "Hello, world!";
        let hash = hash_content(content);
        assert!(verify_hash(content, &hash));
        assert!(!verify_hash("Different content", &hash));
    }

    #[test]
    fn test_consistent_hashing() {
        let content = "Consistent content";
        let hash1 = hash_content(content);
        let hash2 = hash_content(content);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_empty_content() {
        let content = "";
        let hash = hash_content(content);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_whitespace_sensitivity() {
        let content1 = "Hello world";
        let content2 = "Hello  world";
        let content3 = "Hello world\n";

        let hash1 = hash_content(content1);
        let hash2 = hash_content(content2);
        let hash3 = hash_content(content3);

        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash2, hash3);
    }

    #[test]
    fn test_unicode_content() {
        let content = "Hello 世界 🦀";
        let hash = hash_content(content);
        assert!(!hash.is_empty());
        assert!(verify_hash(content, &hash));
    }

    #[test]
    fn test_large_content() {
        let content = "A".repeat(10000);
        let hash = hash_content(&content);
        assert!(!hash.is_empty());
        assert!(verify_hash(&content, &hash));
    }
}
