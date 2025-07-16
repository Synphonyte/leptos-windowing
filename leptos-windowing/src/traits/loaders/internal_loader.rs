use std::{fmt::Debug, ops::Range};

use super::{ExactLoader, LoadedItems, Loader, MemoryLoader, PaginatedCount, PaginatedLoader};

/// This is the trait for the actually used internal loaders.
/// This trait is automatically implemented for all the user facing loader traits.
pub trait InternalLoader<M> {
    /// If this Some(...) then the data will be loaded in chunks of this size.
    /// This is useful for paginated data sources.
    const CHUNK_SIZE: Option<usize> = None;

    /// The type of items that will be loaded.
    type Item;

    /// The type of the query data that will be used to load items.
    ///
    /// Can be used to filter or sort the items for example.
    type Query;

    /// The type of errors that can occur during loading.
    type Error: Debug;

    /// Loads the items respecting the given `range` and `query` together with `CHUNK_SIZE`.
    fn load_items(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> impl Future<Output = Result<LoadedItems<Self::Item>, Self::Error>> {
        let corrected_range = if let Some(chunk_size) = Self::CHUNK_SIZE {
            let Range { start, end } = range;
            let chunk_start = (start / chunk_size) * chunk_size;
            let chunk_end = end.div_ceil(chunk_size) * chunk_size;
            chunk_start..chunk_end
        } else {
            range
        };

        self.load_items_inner(corrected_range, query)
    }

    /// Don't call this directly. Call `load_items` instead.
    ///
    /// Loads the items respecting the given `range` and `query`.
    /// This does not respect `CHUNK_SIZE`.
    fn load_items_inner(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> impl Future<Output = Result<LoadedItems<Self::Item>, Self::Error>>;

    /// The total number of items of this data source.
    ///
    /// Returns `Ok(None)` if unknown (which is the default).
    fn item_count(&self, _query: &Self::Query) -> impl Future<Output = Result<Option<usize>, Self::Error>> {
        async { Ok(None) }
    }
}

pub struct LoaderMarker;

impl<L> InternalLoader<LoaderMarker> for L
where
    L: Loader,
{
    const CHUNK_SIZE: Option<usize> = L::CHUNK_SIZE;

    type Item = L::Item;
    type Query = L::Query;
    type Error = L::Error;

    #[inline]
    async fn load_items_inner(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> Result<LoadedItems<Self::Item>, Self::Error> {
        Loader::load_items(self, range, query).await
    }

    #[inline]
    async fn item_count(&self, query: &Self::Query) -> Result<Option<usize>, Self::Error> {
        Loader::item_count(self, query).await
    }
}

pub struct ExactLoaderMarker;

impl<L> InternalLoader<ExactLoaderMarker> for L
where
    L: ExactLoader,
{
    type Item = L::Item;
    type Query = L::Query;
    type Error = L::Error;

    #[inline]
    async fn load_items_inner(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> Result<LoadedItems<Self::Item>, Self::Error> {
        ExactLoader::load_items(self, range.clone(), query)
            .await
            .map(|items| LoadedItems { items, range })
    }

    #[inline]
    async fn item_count(&self, query: &Self::Query) -> Result<Option<usize>, Self::Error> {
        ExactLoader::item_count(self, query).await
    }
}

pub struct MemoryLoaderMarker;

impl<L> InternalLoader<MemoryLoaderMarker> for L
where
    L: MemoryLoader,
{
    type Item = L::Item;
    type Query = L::Query;
    type Error = ();

    #[inline]
    async fn load_items_inner(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> Result<LoadedItems<Self::Item>, Self::Error> {
        Ok(LoadedItems {
            items: self.load_items(range.clone(), query),
            range,
        })
    }

    #[inline]
    async fn item_count(&self, query: &Self::Query) -> Result<Option<usize>, Self::Error> {
        Ok(Some(MemoryLoader::item_count(self, query)))
    }
}

pub struct PaginatedLoaderMarker;

impl<L> InternalLoader<PaginatedLoaderMarker> for L
where
    L: PaginatedLoader,
{
    const CHUNK_SIZE: Option<usize> = L::CHUNK_SIZE;

    type Item = L::Item;
    type Query = L::Query;
    type Error = L::Error;

    #[inline]
    async fn load_items_inner(
        &self,
        range: Range<usize>,
        query: &Self::Query,
    ) -> Result<LoadedItems<Self::Item>, Self::Error> {
        let Range { start, end } = range;

        debug_assert_eq!(start % L::PAGE_ITEM_COUNT, 0);
        debug_assert_eq!(end - start, L::PAGE_ITEM_COUNT);

        self.load_page(start / L::PAGE_ITEM_COUNT, query)
            .await
            .map(|items| {
                let len = items.len();
                LoadedItems {
                    items,
                    range: start..start + len,
                }
            })
    }

    #[inline]
    async fn item_count(&self, query: &Self::Query) -> Result<Option<usize>, Self::Error> {
        PaginatedLoader::count(self, query).await.map(|count| {
            count.map(|count| match count {
                PaginatedCount::Items(item_count) => item_count,
                PaginatedCount::Pages(page_count) => page_count * L::PAGE_ITEM_COUNT,
            })
        })
    }
}
