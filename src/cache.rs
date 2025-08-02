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
    item_count: Option<usize>,
}

impl<T: Send + Sync + 'static> Default for Cache<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            item_count: None,
        }
    }
}

impl<T: Send + Sync + 'static> Cache<T> {
    /// Create a new store of the cache.
    pub fn new_store() -> Store<Self> {
        Store::new(Self::default())
    }

    #[inline]
    /// Length of the cache.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    /// Resize the cache to the specified length.
    pub fn resize(&mut self, len: usize) {
        self.items.resize(len, ItemState::Placeholder);
    }

    /// Grow the cache size to the specified length.
    pub fn grow(&mut self, len: usize) {
        if self.items.len() < len {
            self.items.resize(len, ItemState::Placeholder);
        }
    }

    /// Marks the specified range of items as loading.
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
            if let Some(mut row) = row.try_write() {
                *row = ItemState::Loading;
            }
        }
    }

    /// Called after the loader has finished loading items.
    ///
    /// This will update the respective range of items with the loaded data (or errors).
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
                    if let Some(mut writer) = this_store.items().try_write() {
                        writer.resize(range.end, ItemState::Placeholder);
                    }
                }

                for (self_row, loaded_row) in this_store
                    .items()
                    .iter_unkeyed()
                    .skip(range.start)
                    .zip(items)
                {
                    if let Some(mut writer) = self_row.try_write() {
                        *writer = ItemState::Loaded(Arc::new(loaded_row));
                    }
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
                    if let Some(mut writer) = row.try_write() {
                        *writer = ItemState::Error(error.clone());
                    }
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

        let start = slice
            .iter()
            .position(do_load_predicate)
            .unwrap_or(slice.len());
        let start = start + range_to_load.start;

        let mut end = if range_to_load.end >= self.items.len() {
            range_to_load.end
        } else {
            slice.iter().rposition(do_load_predicate)? + range_to_load.start + 1
        };

        if let Some(item_count) = self.item_count {
            end = end.min(item_count);
        }

        Some(
            start
                ..end
                    .max(range_to_load.end)
                    .min(self.item_count.unwrap_or(usize::MAX)),
        )
    }

    #[inline]
    /// Sets all items in the cache to the placeholder state.
    pub fn clear(this_store: Store<Self>) {
        this_store.items().write().fill(ItemState::Placeholder);
        this_store.item_count().set(None);
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
        let cache = Cache::<i32>::new_store();

        assert_eq!(cache.read_untracked().missing_range(0..10), Some(0..10));
        assert_eq!(cache.read_untracked().missing_range(5..10), Some(5..10));

        Cache::write_loaded(
            cache,
            Ok(LoadedItems {
                items: (0..5).collect::<Vec<_>>(),
                range: 0..5,
            }),
            0..5,
        );

        assert_eq!(cache.read_untracked().missing_range(0..10), Some(5..10));
        assert_eq!(cache.read_untracked().missing_range(5..10), Some(5..10));
        assert_eq!(cache.read_untracked().missing_range(5..20), Some(5..20));

        Cache::write_loading(cache, 5..9);

        assert_eq!(cache.read_untracked().missing_range(0..10), Some(9..10));
        assert_eq!(cache.read_untracked().missing_range(5..10), Some(9..10));
        assert_eq!(cache.read_untracked().missing_range(5..20), Some(9..20));
    }
}
