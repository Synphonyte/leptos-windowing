use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator};
use std::{
    ops::{Index, Range},
    sync::Arc,
};

use crate::{item_state::ItemState, LoadedItems};

/// This is a cache for items used internally to track
/// which items are already loaded, which are still loading and which are missing.
#[derive(Store, Debug)]
pub struct Cache<T>
where
    T: Send + Sync + 'static,
{
    items: Vec<ItemState<T>>,
}

impl<T: Send + Sync + 'static> Cache<T> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn resize(&mut self, len: usize) {
        self.items.resize(len, ItemState::Placeholder);
    }

    pub fn grow(&mut self, len: usize) {
        if self.items.len() < len {
            self.items.resize(len, ItemState::Placeholder);
        }
    }

    pub fn write_loading(this_store: Store<Self>, range: Range<usize>) {
        if range.end > this_store.items().read().len() {
            this_store
                .items()
                .write()
                .resize(range.end, ItemState::Placeholder);
        }

        for row in &mut this_store
            .items()
            .iter_unkeyed()
            .skip(range.start)
            .take(range.len())
        {
            *row.write() = ItemState::Loading;
        }
    }

    pub fn write_loaded(
        this_store: Store<Self>,
        loading_result: Result<LoadedItems<T>, String>,
        requested_load_range: Range<usize>,
    ) {
        match loading_result {
            Ok(LoadedItems { items, range }) => {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                if range.end > this_store.items().read_untracked().len() {
                    this_store
                        .items()
                        .write()
                        .resize(range.end, ItemState::Placeholder);
                }

                for (self_row, loaded_row) in this_store
                    .items()
                    .iter_unkeyed()
                    .skip(range.start)
                    .zip(items)
                {
                    *self_row.write() = ItemState::Loaded(Arc::new(loaded_row));
                }
            }
            Err(error) => {
                let range = requested_load_range.start
                    ..requested_load_range
                        .end
                        .min(this_store.items().read().len());
                if range.start >= range.end {
                    return;
                }

                for row in this_store.items().iter_unkeyed() {
                    *row.write() = ItemState::Error(error.clone());
                }
            }
        }
    }

    #[inline]
    /// Returns the range of items that are missing from the cache inside the given range.
    ///
    /// Used to know what items should be loaded and which ones are already loaded or in the process of being loaded.
    /// Errored items are not considered missing here.
    pub fn missing_range(&self, range_to_load: Range<usize>) -> Option<Range<usize>> {
        let do_load_predicate = |item: &ItemState<T>| matches!(item, &ItemState::Placeholder);

        if range_to_load.start >= self.items.len() {
            return Some(range_to_load);
        }

        let existing_range_end = self.items.len().min(range_to_load.end);

        let slice = &self.items[range_to_load.start..existing_range_end];

        let start = slice.iter().position(do_load_predicate)?;
        let end = slice.iter().rposition(do_load_predicate)?;

        let start = start + range_to_load.start;
        let end = end + range_to_load.start + 1;

        Some(start..end.max(range_to_load.end))
    }

    #[inline]
    /// Sets all items in the cache to the placeholder state.
    pub fn clear(this_store: Store<Self>) {
        this_store
            .items()
            .update(|items| items.fill(ItemState::Placeholder));
    }
}

impl<T: Sync + Send> Index<Range<usize>> for Cache<T> {
    type Output = [ItemState<T>];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.items[index]
    }
}

impl<T: Send + Sync> Index<usize> for Cache<T> {
    type Output = ItemState<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_range() {
        let cache = Store::new(Cache::<i32>::new());

        assert_eq!(cache.read_untracked().missing_range(0..10), Some(0..10));
        assert_eq!(cache.read_untracked().missing_range(5..10), Some(5..10));

        Cache::write_loaded(
            cache,
            Ok(LoadedItems {
                items: (0..5).into_iter().collect::<Vec<_>>(),
                range: 0..5,
            }),
            0..5,
        );

        assert_eq!(cache.read_untracked().missing_range(0..10), Some(5..10));
        assert_eq!(cache.read_untracked().missing_range(5..10), Some(5..10));

        Cache::write_loading(cache, 5..9);

        assert_eq!(cache.read_untracked().missing_range(0..10), Some(9..10));
        assert_eq!(cache.read_untracked().missing_range(5..10), Some(9..10));
    }
}
