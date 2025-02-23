use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Collection type.
pub type Set<T> = indexmap::IndexSet<T>;

/// Bookmark instance.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bookmark {
    /// Date added.
    #[serde(default = "Timestamp::now")]
    pub date: Timestamp,
    /// Bookmarked URL.
    pub link: String,
    /// Starred by user.
    #[serde(default)]
    pub starred: bool,
    /// Content tags.
    #[serde(default)]
    pub tags: Set<String>,
    /// Content categories.
    #[serde(default)]
    pub categories: Set<String>,
}

/// Bookmark filter query.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    /// Select before date.
    pub until: Option<Timestamp>,
    /// Select after date.
    pub since: Option<Timestamp>,
    /// Starred by user.
    pub starred: Option<bool>,
    /// Content tags.
    #[serde(default)]
    pub tags: Set<String>,
    /// Content categories.
    #[serde(default)]
    pub categories: Set<String>,
}

impl Filter {
    /// Check if a search matches an item.
    pub fn check(&self, item: &Bookmark) -> bool {
        let since = self.since.is_none_or(|since| item.date > since);
        let until = self.until.is_none_or(|until| item.date <= until);
        let starred = self.starred.is_none_or(|starred| item.starred == starred);
        let tags = self.tags.is_subset(&item.tags);
        let cats = self.categories.is_subset(&item.categories);
        since && until && starred && tags && cats
    }
}
