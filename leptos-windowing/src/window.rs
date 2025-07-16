use std::ops::Range;

use leptos::prelude::*;
use reactive_stores::{AtIndex, Store, Subfield};

#[derive(Debug)]
pub struct WindowItem<T: Send + Sync + 'static> {
    pub index: usize,
    pub state: StateAtIndex<T>,
}

pub type StateAtIndex<T> = AtIndex<
    Subfield<Store<Cache<T>, SyncStorage>, Cache<T>, Vec<RwSignal<ItemState<T>>>>,
    Vec<RwSignal<ItemState<T>>>,
>;

use crate::{cache::Cache, item_state::ItemState};

#[derive(Copy, Clone)]
pub struct ItemWindow<T>
where
    T: Send + Sync + 'static,
{
    pub cache: Store<Cache<T>>,
    pub range: Memo<Range<usize>>,
}
