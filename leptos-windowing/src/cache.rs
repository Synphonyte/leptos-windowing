use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator, Subfield};
use std::{
    ops::{Index, Range},
    sync::Arc,
};

use crate::{ItemWindow, LoadedItems, item_state::ItemState};

/// This is a cache for items used internally to track
/// which items are already loaded, which are still loading and which are missing.
pub struct Cache<T>
where
    T: Send + Sync + 'static,
{
    inner: Store<CacheInner<T>>,
    pub(crate) pause_reactive_loading: Callback<()>,
    pub(crate) resume_reactive_loading: Callback<()>,
    pub(crate) is_reactive_loading_active: Signal<bool>,
}

impl<T> Clone for Cache<T>
where
    T: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Cache<T> where T: Send + Sync + 'static {}

#[derive(Store, Debug)]
pub struct CacheInner<T>
where
    T: Send + Sync + 'static,
{
    items: Vec<ItemState<T>>,
    item_count: Option<usize>,
}

impl<T: Send + Sync + 'static> Default for CacheInner<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            item_count: None,
        }
    }
}

impl<T: Send + Sync + 'static> Cache<T> {
    /// Create a new store of the cache.
    pub(crate) fn new() -> Self {
        Self {
            inner: Store::new(CacheInner::default()),
            pause_reactive_loading: (|| {}).into(),
            resume_reactive_loading: (|| {}).into(),
            is_reactive_loading_active: Signal::stored(true),
        }
    }

    #[inline]
    /// After calling this, changes to the cache will not trigger (re)loading with the loader
    pub fn pause_reactive_loading(&self) {
        self.pause_reactive_loading.run(());
    }

    #[inline]
    /// After calling this, changes to the cache will resume triggering (re)loading with the loader
    pub fn resume_reactive_loading(&self) {
        self.resume_reactive_loading.run(());
    }

    /// Pauses reactive loading before executing `f` and then resumes reactive loading if it hadn't been
    /// paused before.
    pub fn with_reactive_loading_paused<O>(&self, f: impl FnOnce() -> O) -> O {
        let is_active = self.is_reactive_loading_active.get_untracked();
        self.pause_reactive_loading();

        let ret = f();

        if is_active {
            self.resume_reactive_loading();
        }

        ret
    }

    #[inline]
    pub fn track(&self) {
        self.inner.track();
    }

    #[inline]
    /// Length of the items of cache.
    pub fn len(&self) -> usize {
        self.inner.items().read().len()
    }

    #[inline]
    /// True when there are no items in the cache.
    pub fn is_empty(&self) -> bool {
        self.inner.items().read().is_empty()
    }

    #[inline]
    /// Item count subfield
    pub fn item_count(&self) -> Subfield<Store<CacheInner<T>>, CacheInner<T>, Option<usize>> {
        self.inner.item_count()
    }

    #[inline]
    pub fn items(&self) -> Subfield<Store<CacheInner<T>>, CacheInner<T>, Vec<ItemState<T>>> {
        self.inner.items()
    }

    #[inline]
    /// Resize the cache to the specified length.
    pub fn resize(&mut self, len: usize) {
        self.inner
            .items()
            .write()
            .resize(len, ItemState::Placeholder);
    }

    /// Grow the cache size to the specified length.
    pub fn grow(&mut self, len: usize) {
        if self.inner.items().read().len() < len {
            self.inner
                .items()
                .write()
                .resize(len, ItemState::Placeholder);
        }
    }

    /// Marks the specified range of items as loading.
    pub fn write_loading(&self, range: Range<usize>) {
        if range.end > self.inner.items().read().len() {
            self.inner
                .items()
                .write()
                .resize(range.end, ItemState::Placeholder);
        }

        for row in &mut self
            .inner
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
        &self,
        loading_result: Result<LoadedItems<T>, String>,
        requested_load_range: Range<usize>,
    ) {
        match loading_result {
            Ok(LoadedItems { items, range }) => {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                if range.end > self.inner.items().read_untracked().len()
                    && let Some(mut writer) = self.inner.items().try_write()
                {
                    writer.resize(range.end, ItemState::Placeholder);
                }

                for (self_row, loaded_row) in self
                    .inner
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
                        .min(self.inner.items().read().len());
                if range.start >= range.end {
                    return;
                }

                for row in self.inner.items().iter_unkeyed() {
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

        if range_to_load.end <= range_to_load.start {
            return None;
        }

        if range_to_load.start >= self.inner.items().read().len() {
            return Some(range_to_load);
        }

        let existing_range_end = self.inner.items().read().len().min(range_to_load.end);

        let slice = &self.inner.items().read()[range_to_load.start..existing_range_end];

        let start = slice
            .iter()
            .position(do_load_predicate)
            .unwrap_or(slice.len());
        let start = start + range_to_load.start;

        let mut end = if range_to_load.end >= self.inner.items().read().len() {
            range_to_load.end
        } else {
            slice.iter().rposition(do_load_predicate)? + range_to_load.start + 1
        };

        if let Some(item_count) = self.inner.item_count().get() {
            end = end.min(item_count);
        }

        if end <= start {
            return None;
        }

        Some(
            start
                ..end
                    .max(range_to_load.end)
                    .min(self.inner.item_count().get().unwrap_or(usize::MAX)),
        )
    }

    #[inline]
    /// Sets all items in the cache to the placeholder state.
    pub fn clear(&self) {
        self.inner.items().write().fill(ItemState::Placeholder);
        self.inner.item_count().set(None);
    }

    /// Updates an item in the cache.
    ///
    /// This doesn't trigger a reload.
    ///
    /// The user is responsible for updating the data source accordingly.
    pub fn update_item(&self, index: usize, new: T) {
        self.with_reactive_loading_paused(|| {
            *self.inner.items().at_unkeyed(index).write() = ItemState::Loaded(Arc::new(new));
        });
    }

    /// Removes the item at the given index from the cache and updates the item count.
    ///
    /// This doesn't trigger a reload.
    ///
    /// The user is responsible for updating the data source accordingly.
    pub fn remove_item(&self, index: usize) {
        self.with_reactive_loading_paused(|| {
            self.inner.items().write().remove(index);

            if let Some(len) = self.inner.item_count().get_untracked() {
                self.inner.item_count().set(Some(len - 1));
            }
        });
    }

    /// Inserts an item at the given index in the cache and updates the item count.
    ///
    /// This doesn't trigger a reload.
    ///
    /// The user is responsible for updating the data source accordingly.
    pub fn insert_item(&self, index: usize, new: T) {
        self.with_reactive_loading_paused(|| {
            self.inner
                .items()
                .write()
                .insert(index, ItemState::Loaded(Arc::new(new)));

            if let Some(len) = self.inner.item_count().get_untracked() {
                self.inner.item_count().set(Some(len + 1));
            }
        });
    }
}

impl<T: Sync + Send> Index<Range<usize>> for CacheInner<T> {
    type Output = [ItemState<T>];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.items[index]
    }
}

impl<T: Send + Sync> Index<usize> for CacheInner<T> {
    type Output = ItemState<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

/// This can be used to get write access to the cache.
pub struct CacheController<T>
where
    T: Send + Sync + 'static,
{
    cache: StoredValue<Option<Cache<T>>>,
}

impl<T> Clone for CacheController<T>
where
    T: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for CacheController<T> where T: Send + Sync + 'static {}

impl<T> Default for CacheController<T>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            cache: StoredValue::new(None),
        }
    }
}

impl<T> CacheController<T>
where
    T: Send + Sync + 'static,
{
    /// You know what this does.
    pub fn new() -> Self {
        Default::default()
    }

    /// This is called in the components to init the connection to the cache
    pub fn init_with_item_window(&self, window: ItemWindow<T>) {
        self.cache.set_value(Some(window.cache));
    }

    /// Updates an item in the cache.
    ///
    /// This doesn't trigger a reload.
    ///
    /// The user is responsible for updating the data source accordingly.
    pub fn update_item(&self, index: usize, new: T) {
        if let Some(cache) = self.cache.get_value() {
            cache.update_item(index, new);
        } else {
            leptos::logging::error!(
                "Update item is called on a cache controller before the controller has been initialized."
            )
        }
    }

    /// Removes the item at the given index from the cache and updates the item count.
    ///
    /// This doesn't trigger a reload.
    ///
    /// The user is responsible for updating the data source accordingly.
    pub fn remove_item(&self, index: usize) {
        if let Some(cache) = self.cache.get_value() {
            cache.remove_item(index);
        } else {
            leptos::logging::error!(
                "Remove item is called on a cache controller before the controller has been initialized."
            )
        }
    }

    /// Inserts an item at the given index in the cache and updates the item count.
    ///
    /// This doesn't trigger a reload.
    ///
    /// The user is responsible for updating the data source accordingly.
    pub fn insert_item(&self, index: usize, new: T) {
        if let Some(cache) = self.cache.get_value() {
            cache.insert_item(index, new);
        } else {
            leptos::logging::error!(
                "Insert item is called on a cache controller before the controller has been initialized."
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_range() {
        let cache = Cache::<i32>::new();

        assert_eq!(cache.missing_range(0..10), Some(0..10));
        assert_eq!(cache.missing_range(5..10), Some(5..10));

        cache.write_loaded(
            Ok(LoadedItems {
                items: (0..5).collect::<Vec<_>>(),
                range: 0..5,
            }),
            0..5,
        );

        assert_eq!(cache.missing_range(0..10), Some(5..10));
        assert_eq!(cache.missing_range(5..10), Some(5..10));
        assert_eq!(cache.missing_range(5..20), Some(5..20));

        cache.write_loading(5..9);

        assert_eq!(cache.missing_range(0..10), Some(9..10));
        assert_eq!(cache.missing_range(5..10), Some(9..10));
        assert_eq!(cache.missing_range(5..20), Some(9..20));
    }
}
