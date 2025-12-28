//! Pagination types for API responses.

use serde::{Deserialize, Serialize};

/// A paginated response using page numbers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Total count of items.
    pub count: Option<i64>,
    /// URL for the next page.
    pub next: Option<String>,
    /// URL for the previous page.
    pub previous: Option<String>,
    /// The results for this page.
    pub results: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    /// Returns true if there is a next page.
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Returns true if there is a previous page.
    pub fn has_previous(&self) -> bool {
        self.previous.is_some()
    }

    /// Returns the total number of pages (if count is available).
    pub fn total_pages(&self, page_size: usize) -> Option<usize> {
        self.count.map(|c| (c as usize).div_ceil(page_size))
    }
}

/// A cursor-based paginated response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPaginatedResponse<T> {
    /// Cursor for the next page.
    pub next: Option<String>,
    /// Cursor for the previous page.
    pub previous: Option<String>,
    /// The results for this page.
    pub results: Vec<T>,
}

impl<T> CursorPaginatedResponse<T> {
    /// Returns true if there is a next page.
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Returns true if there is a previous page.
    pub fn has_previous(&self) -> bool {
        self.previous.is_some()
    }

    /// Extracts the cursor value from the next URL.
    pub fn next_cursor(&self) -> Option<String> {
        self.next.as_ref().and_then(|url| {
            url::Url::parse(url).ok().and_then(|u| {
                u.query_pairs()
                    .find(|(k, _)| k == "cursor")
                    .map(|(_, v)| v.to_string())
            })
        })
    }

    /// Extracts the page number from the next URL.
    pub fn next_page(&self) -> Option<i32> {
        self.next.as_ref().and_then(|url| {
            url::Url::parse(url).ok().and_then(|u| {
                u.query_pairs()
                    .find(|(k, _)| k == "page")
                    .and_then(|(_, v)| v.parse().ok())
            })
        })
    }
}
