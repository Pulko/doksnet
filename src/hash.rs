use blake3;

/// Generate a hash for the given content using Blake3
pub fn hash_content(content: &str) -> String {
    let hash = blake3::hash(content.as_bytes());
    hash.to_hex().to_string()
}

/// Verify if the content matches the expected hash
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
        assert_eq!(hash.len(), 64); // Blake3 produces 256-bit (32 byte) hashes, 64 hex characters
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
} 