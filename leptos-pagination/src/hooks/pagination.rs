use std::fmt::Debug;

use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos_windowing::{
    use_load_on_demand::{use_load_on_demand, UseLoadOnDemandResult},
    InternalLoader, ItemWindow,
};
use reactive_stores::Store;

use crate::{PaginationState, PaginationStateStoreFields};

/// Hook for the pagination logic.
///
/// This handles loading items on-demand from the data source and caching them.
///
/// It returns an [`ItemWindow`] that is in effect a signal of the items to display.
///
/// ## Usage
///
/// ```
/// # use std::ops::Range;
/// #
/// # use leptos_windowing::pagination::{use_pagination, use_pagination_controls, UsePaginationOptions, UsePaginationControlsOptions, PaginationState};
/// # use leptos_windowing::MemoryLoader;
/// #
/// let state = PaginationState::new_store();
///
/// pub struct ExampleItem {
///     num: usize,
/// }
///
/// // Implement Loader for example data
/// pub struct ExampleLoader;
///
/// impl MemoryLoader for ExampleLoader {
///     type Item = ExampleItem;
///     type Query = ();
///
///     fn load_items(&self, range: Range<usize>, query: &Self::Query) -> Vec<Self::Item> {
///         // Generate example data
///         range.map(|i| ExampleItem { num: i }).collect()
///     }
///
///     fn item_count(&self, query: &Self::Query) -> usize {
///         // Let's say we have 1000 items in total
///         1000
///     }
/// }
///
/// // See PaginatedFor for how to build a pagination component with the returned window from this hook.
/// let window = use_pagination(
///     state,
///     ExampleLoader,
///     (),
///     20, // items per page
///     UsePaginationOptions::default(),
/// );
///
/// // Use this to control the pagination
/// let pagination_controls = use_pagination_controls(state, UsePaginationControlsOptions::default());
/// ```
///
/// ## Paramters
///
/// - `state`: The pagination state. Used to communicate between the pagination controls and this component.
/// - `loader`: The loader used to load items from the data source.
/// - `item_count_per_page`: The number of items to display per page.
/// - `options`: Additional options for the pagination logic.
#[must_use]
pub fn use_pagination<T, L, Q, M>(
    state: Store<PaginationState>,
    loader: L,
    query: impl Into<Signal<Q>>,
    item_count_per_page: impl Into<Signal<usize>>,
    options: UsePaginationOptions,
) -> ItemWindow<T>
where
    T: Send + Sync + 'static,
    L: InternalLoader<M, Item = T, Query = Q> + 'static,
    Q: Send + Sync + 'static,
{
    let UsePaginationOptions {
        overscan_page_count,
    } = options;

    let item_count_per_page = item_count_per_page.into();

    let item_count = RwSignal::new(None::<usize>);

    Effect::new(move || {
        if let Some(item_count) = item_count.get() {
            state
                .page_count()
                .set(Some(item_count.div_ceil(item_count_per_page.get())));
        }
    });

    let start_index_to_load = Signal::derive(move || {
        let current_page = state.current_page().get();
        current_page.saturating_sub(overscan_page_count) * item_count_per_page.get()
    });

    let end_index_to_load = Signal::derive(move || {
        let current_page = state.current_page().get();
        (current_page + overscan_page_count) * item_count_per_page.get()
    });

    let range_to_load = Memo::new(move |_| {
        let start_index = start_index_to_load.get();
        let end_index = end_index_to_load.get();

        start_index..end_index
    });

    let range_to_display = Memo::new(move |_| {
        let item_count_per_page = item_count_per_page.get();
        let start_index = state.current_page().get() * item_count_per_page;
        let end_index = start_index + item_count_per_page;

        start_index..end_index
    });

    let UseLoadOnDemandResult {
        item_count_result,
        item_window,
    } = use_load_on_demand(range_to_load, range_to_display, loader, query);

    Effect::new(move || {
        match &*item_count_result.read() {
            Ok(None) => {
                *state.page_count_error().write() =
                    Some("Data source didn't provide an item/page count".to_string())
            }
            Ok(Some(count)) => {
                // This sets the page_count. See effect above.
                item_count.set(Some(*count));
                *state.page_count_error().write() = None;
            }
            Err(err) => {
                *state.page_count_error().write() =
                    Some(format!("Error fetching item/page count: {err:?}"))
            }
        }
    });

    item_window
}

#[derive(Debug, Clone, DefaultBuilder)]
pub struct UsePaginationOptions {
    /// How many pages to load before and after the current page.
    ///
    /// A value of 1 means that the current page as well as the one before and after will be loaded.
    /// Defaults to 1.
    overscan_page_count: usize,
}

impl Default for UsePaginationOptions {
    fn default() -> Self {
        Self {
            overscan_page_count: 1,
        }
    }
}
