use std::{fmt::Debug, ops::Range};

use leptos::{prelude::*, reactive::spawn_local};

use crate::{cache::{Cache, CacheStoreFields}, InternalLoader, ItemWindow};

/// Load items on demand and cache them.
///
/// Underlying functionality of [`use_pagination`] and [`use_virtualization`].
/// You most probably don't want to use this directly but either [`use_pagination`] or [`use_virtualization`].
///
/// ## Params
/// - `load_range`: A signal of the range of items to load. This has to include the `display_range`. Control the range of items to load and cache.
/// - `display_range`: A signal of the range of items to display. This will be used for the returned `ItemWindow`.
/// - `loader`: The loader to use for loading items.
/// - `query`: A signal of the query to use for loading items.
///
/// ## Returns
///
/// A tuple containing:
/// - `Signal<Result<Option<usize>, E>>`: A signal of the total number of items.
///   This will be either:
///   - `Ok(Some(n))`: The total number of items.
///   - `Ok(None)`: The total number of items is unknown.
///   - `Err(e)`: An error occurred while loading the total number of items.
/// - `ItemWindow<T>`: A window of items that can be used to render a list/table of items.
pub fn use_load_on_demand<T, L, Q, E, M>(
    range_to_load: impl Into<Signal<Range<usize>>>,
    range_to_display: impl Into<Signal<Range<usize>>>,
    loader: L,
    query: impl Into<Signal<Q>>,
) -> UseLoadOnDemandResult<T, E>
where
    T: Send + Sync + 'static,
    L: InternalLoader<M, Item = T, Query = Q, Error = E> + 'static,
    Q: Send + Sync + 'static,
    E: Debug + 'static,
{
    let range_to_load = range_to_load.into();
    let range_to_display = range_to_display.into();

    let cached_range_to_display = RwSignal::new(0..0);

    let cache = Cache::new_store();

    let loader = Signal::stored_local(loader);
    let query = query.into();

    let item_count_result = RwSignal::new_local(Ok(None));

    let set_item_count = move |count: Result<Option<usize>, E>| {
        cache.item_count().set(count.as_ref().ok().flatten().copied());
        item_count_result.set(count);
    };

    // Load item count
    Effect::new(move || {
        query.track();
        leptos::logging::log!("Loading item count");
        spawn_local(async move {
            let count = loader.read().item_count(&*query.read_untracked()).await;
            leptos::logging::log!("Item count loaded: {count:?}");
            set_item_count(count);
        });
    });

    let reload_trigger = Trigger::new();

    // Clear cache
    Effect::new(move || {
        query.track();
        Cache::clear(cache);
        reload_trigger.notify();
    });

    // Load items
    Effect::new(move || {
        // we don't need to track the query here because it triggers cache invalidation which triggers reload_trigger
        reload_trigger.track();

        let missing_range = cache.read().missing_range(range_to_load.get());

        if let Some(missing_range) = missing_range {
            Cache::write_loading(cache, missing_range.clone());

            spawn_local(async move {
                let result = loader
                    .read()
                    .load_items(missing_range.clone(), &*query.read_untracked())
                    .await;

                if let Ok(loaded_items) = &result {
                    if loaded_items.range.end < missing_range.end {
                        set_item_count(Ok(Some(loaded_items.range.end)));
                    }
                }

                // TODO : check if still relevant or other loading has started

                Cache::write_loaded(cache, result.map_err(|e| format!("{e:?}")), missing_range);
            });
        }

        // Make sure that the cache is filled and then update the display range
        let Range { start, end } = range_to_display.get();
        cached_range_to_display.set(start..end.min(cache.item_count().get().unwrap_or(usize::MAX)));
    });

    UseLoadOnDemandResult {
        item_count_result: item_count_result.into(),
        item_window: ItemWindow {
            cache,
            range: cached_range_to_display.into(),
        },
    }
}

/// Return type of [`use_load_on_demand`].
pub struct UseLoadOnDemandResult<T, E>
where
    T: Send + Sync + 'static,
    E: Debug + 'static,
{
    pub item_count_result: Signal<Result<Option<usize>, E>, LocalStorage>,
    pub item_window: ItemWindow<T>,
}
