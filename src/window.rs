use std::ops::Range;

use leptos::prelude::*;
use reactive_stores::Store;

use crate::cache::Cache;

/// This is bascially a signal of a slice of the internal cache.
///
/// This is returned by [`use_pagination`] and [`use_virtualization`](http://TO.DO).
#[derive(Copy, Clone)]
pub struct ItemWindow<T>
where
    T: Send + Sync + 'static,
{
    pub cache: Store<Cache<T>>,
    pub range: Signal<Range<usize>>,
}
