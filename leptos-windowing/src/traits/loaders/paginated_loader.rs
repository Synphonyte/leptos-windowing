use std::fmt::Debug;

use crate::SortMode;

/// Loader trait for loading items on-demand from a paginated data source.
///
/// Please note that this is independent of if you use pagination or virtualization in your UI.
/// This just referrs to the data source. So if you have an API for example, that returns the
/// data paginated, then this is for you.
pub trait PaginatedLoader {
    /// How many rows per page
    const PAGE_ITEM_COUNT: usize;

    /// The type of items that will be loaded.
    type Item;

    /// The type of errors that can occur during loading.
    type Error: Debug;

    /// Get all data items specified by the page index (starts a 0).
    ///
    /// If you return less than `PAGE_ITEM_COUNT` rows, it is assumed that the end of the
    /// data has been reached.
    fn load_page(
        &self,
        page_index: usize,
        sorting: &[(usize, SortMode)],
    ) -> impl Future<Output = Result<Vec<Self::Item>, Self::Error>>;

    /// The total number of items of this data source.
    ///
    /// Returns `Ok(None)` if unknown (which is the default).
    fn count(&self) -> impl Future<Output = Result<Option<PaginatedCount>, Self::Error>> {
        async { Ok(None) }
    }
}

/// Return type of [`PaginatedLoader::count`].
pub enum PaginatedCount {
    /// If your data source tells you how many pages there are, then use this.
    Pages(usize),

    /// If your data source tells you how many items there are, then use this.
    Items(usize),
}
