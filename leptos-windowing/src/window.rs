use std::{ops::Range, sync::Arc};

use leptos::prelude::*;

use crate::cache::Cache;

/// This is bascially a signal of a slice of the internal cache.
///
/// This is returned by `use_pagination` and `use_virtualization`.
pub struct ItemWindow<T>
where
    T: Send + Sync + 'static,
{
    pub cache: Cache<T>,
    pub range: Signal<Range<usize>>,
}

impl<T> Clone for ItemWindow<T>
where
    T: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ItemWindow<T> where T: Send + Sync + 'static {}

impl<T> ItemWindow<T>
where
    T: Send + Sync + 'static,
{
    /// Updates an item in the cache at the specified index.
    ///
    /// The user is responsible to make sure that the data source is updated accordingly.
    #[inline]
    pub fn update_item(&self, index: usize, item: T) {
        self.cache.update_item(index, item);
    }

    /// Inserts an item into the cache at the specified index.
    ///
    /// The user is responsible to make sure that the data source is updated accordingly.
    #[inline]
    pub fn insert_item(&self, index: usize, item: T) {
        self.cache.insert_item(index, item);
    }

    /// Removes an item from the cache at the specified index.
    ///
    /// The user is responsible to make sure that the data source is updated accordingly.
    #[inline]
    pub fn remove_item(&self, index: usize) {
        self.cache.remove_item(index);
    }
}

/// Item in a [`ItemWindow`].
pub struct WindowItem<T>
where
    T: Send + Sync + 'static,
{
    pub index: usize,
    pub data: Arc<T>,
    cache: Cache<T>,
}

impl<T> Clone for WindowItem<T>
where
    T: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            data: Arc::clone(&self.data),
            cache: self.cache,
        }
    }
}

impl<T> WindowItem<T>
where
    T: Send + Sync + 'static,
{
    /// Creates a new `WindowItem` with the given index, data, and item window.
    pub fn new(index: usize, data: Arc<T>, window: &ItemWindow<T>) -> Self {
        Self {
            index,
            data,
            cache: window.cache,
        }
    }

    /// Updates the data in the cache associated with the item.
    ///
    /// The user is responsible for updating the data source accordingly.
    #[inline]
    pub fn update(&self, new: T) {
        self.cache.update_item(self.index, new);
    }

    /// Removes the item from the cache.
    ///
    /// The user is responsible for updating the data source accordingly.
    #[inline]
    pub fn remove(&self) {
        self.cache.remove_item(self.index);
    }

    /// Inserts an item before the current item in the cache.
    ///
    /// The user is responsible for updating the data source accordingly.
    #[inline]
    pub fn insert_before(&self, item: T) {
        self.cache.insert_item(self.index, item);
    }

    /// Inserts an item after the current item in the cache.
    ///
    /// The user is responsible for updating the data source accordingly.
    #[inline]
    pub fn insert_after(&self, item: T) {
        self.cache.insert_item(self.index + 1, item);
    }

    /// Inserts an item at the specified index in the cache.
    ///
    /// The user is responsible for updating the data source accordingly.
    #[inline]
    pub fn insert(&self, index: usize, item: T) {
        self.cache.insert_item(index, item);
    }
}
