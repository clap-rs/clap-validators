/// Ensure a string is long enough
pub fn is_longer_than(s: String, l: usize) -> Result<(), String> {
    let len = s.chars().count();
    if len >= l {
    	return Ok(())
    }
    Err(format!("{} has {} chars so it is less than {} chars", s, len, l))
}

/// Ensure a string is short enough
pub fn is_shorter_than(s: String, l: usize) -> Result<(), String> {
    let len = s.chars().count();
    if len < l {
    	return Ok(())
    }
    Err(format!("{} has {} chars so it is more than {} chars", s, len, l))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_longer_than_true() {
        assert!(is_longer_than("ラウトは難しいです！".to_string(), 10).is_ok());
    }

    #[test]
    fn test_is_longer_than_false() {
        assert!(is_longer_than("short".to_string(), 10).is_err());
    }

    #[test]
    fn test_is_shorter_than_true() {
        assert!(is_shorter_than("ラウトは難しいです！".to_string(), 30).is_ok());
    }

    #[test]
    fn test_is_shorter_than_false() {
        assert!(is_shorter_than("short".to_string(), 3).is_err());
    }
 }