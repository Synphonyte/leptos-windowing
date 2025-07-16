mod cache;
pub mod item_state;
pub mod pagination;
mod traits;
mod window;

pub use traits::*;
pub use window::*;

use serde::{Deserialize, Serialize};

/// Type of sorting of a column
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SortMode {
    Ascending,
    Descending,
    None,
}

impl SortMode {
    /// Returns the default CSS class name
    pub fn as_class(&self) -> &'static str {
        match self {
            SortMode::Ascending => "sort-asc",
            SortMode::Descending => "sort-desc",
            _ => "",
        }
    }

    /// Returns the SQL sort order (ASC or DESC) or `None` if `ColumnSort::None`.
    pub fn as_sql(&self) -> Option<&'static str> {
        match self {
            SortMode::Ascending => Some("ASC"),
            SortMode::Descending => Some("DESC"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(SortMode::Ascending, SortMode::Ascending);
    }
}
