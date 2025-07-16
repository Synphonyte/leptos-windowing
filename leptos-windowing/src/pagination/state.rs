use std::collections::VecDeque;

use leptos::prelude::*;
use reactive_stores::Store;

use crate::SortMode;

/// The state of pagination.
///
/// Used as a reactive store to communicate between control and display components.
#[derive(Store, Clone, Debug, PartialEq, Eq)]
pub struct PaginationState {
    /// The current page number. Counting starts from 0.
    pub current_page: usize,
    /// The total number of pages or None initially or if the count could not be determined.
    pub page_count: Option<usize>,
    /// The error message if the page count could not be determined.
    pub page_count_error: Option<String>,

    /// The sorting order of the data. The first element is the index of the for example column/row.
    /// The second element is the sorting mode.
    pub sorting: VecDeque<(usize, SortMode)>,

    reload_trigger: usize,
}

impl PaginationState {
    pub fn new() -> Store<Self> {
        Store::new(Self {
            current_page: 0,
            page_count: None,
            page_count_error: None,
            sorting: VecDeque::new(),
            reload_trigger: 0,
        })
    }

    /// If possible, move to the next page.
    pub fn next(this_store: Store<Self>) {
        if !Self::is_last_page(this_store) {
            this_store.current_page().update(|cp| *cp += 1);
        }
    }

    /// If possible, move to the previous page.
    pub fn prev(this_store: Store<Self>) {
        if this_store.current_page().get() > 0 {
            this_store.current_page().update(|cp| *cp -= 1);
        }
    }

    pub fn is_first_page(this_store: Store<Self>) -> bool {
        this_store.current_page().get() == 0
    }

    pub fn is_last_page(this_store: Store<Self>) -> bool {
        if let Some(page_count) = this_store.page_count().get() {
            this_store.current_page().get() >= page_count.saturating_sub(1)
        } else {
            false
        }
    }

    /// Call this to clear the cache and reload the data.
    pub fn trigger_reload(this_store: Store<Self>) {
        this_store
            .reload_trigger()
            .update(|rt| *rt = rt.wrapping_add(1));
    }
}
