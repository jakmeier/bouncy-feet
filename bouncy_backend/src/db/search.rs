#[derive(Clone, Debug)]
pub struct DbSearchCheckedString(String);

#[derive(Debug, thiserror::Error)]
pub enum DbSearchStringError {
    #[error("Search string must be at least {min} characters long but was {was}")]
    TooShort { min: usize, was: usize },
    #[error("Search string must be at most {max} characters long but was {was}")]
    TooLong { max: usize, was: usize },
}

impl DbSearchCheckedString {
    const MIN_LEN: usize = 2;
    const MAX_LEN: usize = 100;

    pub fn check(raw: impl Into<String>) -> Result<Self, DbSearchStringError> {
        let s = raw.into();
        let s = s.trim();
        let s: String = s.split_whitespace().collect::<Vec<_>>().join(" ");

        if s.len() < Self::MIN_LEN {
            return Err(DbSearchStringError::TooShort {
                min: Self::MIN_LEN,
                was: s.len(),
            });
        }
        if s.len() > Self::MAX_LEN {
            return Err(DbSearchStringError::TooLong {
                max: Self::MAX_LEN,
                was: s.len(),
            });
        }

        let escaped = s
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_");

        Ok(Self(escaped))
    }
}

impl AsRef<str> for DbSearchCheckedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for DbSearchCheckedString {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str) -> Result<String, DbSearchStringError> {
        DbSearchCheckedString::check(s).map(|v| v.0)
    }

    #[test]
    fn exact_minimum_length_accepted() {
        assert!(DbSearchCheckedString::check("ab").is_ok());
    }

    #[test]
    fn exact_maximum_length_accepted() {
        assert!(DbSearchCheckedString::check("a".repeat(100)).is_ok());
    }

    #[test]
    fn normal_string_passes_through() {
        assert_eq!(check("alice").unwrap(), "alice");
    }

    #[test]
    fn leading_and_trailing_whitespace_is_trimmed() {
        assert_eq!(check("  alice  ").unwrap(), "alice");
    }

    #[test]
    fn internal_whitespace_is_collapsed() {
        assert_eq!(check("alice   smith").unwrap(), "alice smith");
    }

    #[test]
    fn empty_string_rejected() {
        assert!(matches!(
            DbSearchCheckedString::check(""),
            Err(DbSearchStringError::TooShort { was: 0, .. })
        ));
    }

    #[test]
    fn one_character_rejected() {
        assert!(matches!(
            DbSearchCheckedString::check("a"),
            Err(DbSearchStringError::TooShort { was: 1, .. })
        ));
    }

    #[test]
    fn whitespace_only_rejected() {
        assert!(matches!(
            DbSearchCheckedString::check("   "),
            Err(DbSearchStringError::TooShort { was: 0, .. })
        ));
    }

    #[test]
    fn string_over_max_rejected() {
        assert!(matches!(
            DbSearchCheckedString::check("b".repeat(101)),
            Err(DbSearchStringError::TooLong { was: 101, .. })
        ));
    }

    #[test]
    fn percent_is_escaped() {
        assert_eq!(check("100%").unwrap(), "100\\%");
    }

    #[test]
    fn underscore_is_escaped() {
        assert_eq!(check("snake_case").unwrap(), "snake\\_case");
    }

    #[test]
    fn backslash_is_escaped() {
        assert_eq!(check("C:\\path").unwrap(), "C:\\\\path");
    }

    #[test]
    fn multiple_special_chars_all_escaped() {
        assert_eq!(check("a%b_c\\d").unwrap(), "a\\%b\\_c\\\\d");
    }

    #[test]
    fn length_check_runs_before_escaping() {
        assert!(DbSearchCheckedString::check("%".repeat(100)).is_ok());
    }
}
