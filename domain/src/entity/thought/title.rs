#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(String);

impl Title{
    pub const fn new(title:String) -> Self {
        Self(title)
    }
}

impl From<Title> for String {
    fn from(from: Title) -> Self {
        from.0
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// withing domain, we define doamin wide artifacts
// but the actual verification is done in the out layers

const MAX_TITLE_LEN: usize = 80;
const MIN_TITLE_LEN: usize = 3;

#[derive(Debug)]
pub struct TitleConstraints;

impl TitleConstraints {
    pub const fn min_len() -> usize {
        MIN_TITLE_LEN
    }

    pub const fn max_len() -> usize {
        MAX_TITLE_LEN
    }
}

#[cfg(test)]
mod tests {
    use super::Title;

    #[test]
    fn get_title() {
        let title = Title::new("Lord of the Rings".to_owned());
        let title_string = String::from(title);
        assert_eq!(title_string, "Lord of the Rings".to_string())
    }
}