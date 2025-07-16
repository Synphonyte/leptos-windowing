use default_struct_builder::DefaultBuilder;
use leptos::{prelude::*, reactive::spawn_local};
use reactive_stores::Store;

use crate::{
    InternalLoader, ItemWindow,
    cache::Cache,
    pagination::{PaginationState, PaginationStateStoreFields},
};

/// Hook for the pagination logic.
///
/// This handles loading items on-demand from the data source and caching them.
///
/// It returns an [`ItemWindow`] that is in effect a signal of the items to display.
///
/// ## Usage
///
/// ```
/// # use leptos_windowing::{use_pagination, use_pagination_controls, UsePaginationOptions, UsePaginationControlsOptions};
/// #
/// let state = PaginationState::default();
///
/// // This is the simplest way to implement a loader for a `Vec<ExampleItem>`.
/// #[derive(MemoryLoader)]
/// pub struct ExampleItem {
///     num: usize,
/// }
///
/// // Generate example data
/// let data: Vec<ExampleItem> = (0..100).map(|i| ExampleItem { num: i }).collect();
///
/// // Use `window.iter()` with `<For>` to display the items.
/// let window = use_pagination(
///     state,
///     data,
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
pub fn use_pagination<T, L, M>(
    state: Store<PaginationState>,
    loader: L,
    item_count_per_page: impl Into<Signal<usize>>,
    options: UsePaginationOptions,
) -> ItemWindow<T>
where
    T: Send + Sync + 'static,
    L: InternalLoader<M, Item = T> + 'static,
{
    let UsePaginationOptions {
        overscan_page_count,
    } = options;

    let cache = Store::new(Cache::new());
    let loader = Signal::stored_local(loader);

    let item_count_per_page = item_count_per_page.into();

    let item_count = RwSignal::new(None::<usize>);

    Effect::new(move || {
        if let Some(item_count) = item_count.get() {
            state
                .page_count()
                .set(Some(item_count.div_ceil(item_count_per_page.get())));
        }
    });

    // Load item count
    spawn_local(async move {
        let count = loader.read().item_count().await;

        match count {
            Ok(None) => {
                *state.page_count_error().write() =
                    Some("Data source didn't provide an item/page count".to_string())
            }
            Ok(Some(count)) => {
                // This already sets the page_count. See effect above.
                item_count.set(Some(count));
            }
            Err(err) => {
                *state.page_count_error().write() =
                    Some(format!("Error fetching item/page count: {err:?}"))
            }
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
        let mut end_index = end_index_to_load.get();

        if let Some(item_count) = item_count.get() {
            end_index = end_index.min(item_count);
        }

        start_index..end_index
    });

    // Load items
    Effect::new(move |prev_clear_state| {
        if Some((state.sorting().get(), state.reload_trigger().get())) != prev_clear_state {
            Cache::clear(cache);
        }

        let missing_range = cache.read().missing_range(range_to_load.get());

        if let Some(missing_range) = missing_range {
            Cache::write_loading(cache, missing_range.clone());

            spawn_local(async move {
                let result = loader
                    .read()
                    .load_items(
                        missing_range.clone(),
                        &state.sorting().read_untracked().iter().copied().collect::<Vec<_>>(),
                    )
                    .await;

                if let Ok(loaded_items) = &result {
                    if loaded_items.range.end < missing_range.end {
                        item_count.set(Some(loaded_items.range.end));
                    }
                }

                Cache::write_loaded(cache, result.map_err(|e| format!("{e:?}")), missing_range);
            });
        }

        (state.sorting().get(), state.reload_trigger().get())
    });

    let window_range = Memo::new(move |_| {
        let item_count = item_count_per_page.get();
        let start_index = state.current_page().get() * item_count;

        let display_range = start_index..start_index + item_count;

        cache.write().grow(display_range.end);

        display_range
    });

    ItemWindow {
        cache,
        range: window_range,
    }
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
