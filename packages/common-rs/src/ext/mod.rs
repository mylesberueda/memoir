pub trait SlugifyExt
where
    Self: AsRef<str>,
{
    /// Generate a URL-friendly slug from a name
    fn slugify(&self) -> String {
        self.as_ref()
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

impl SlugifyExt for &str {}

impl SlugifyExt for String {}
