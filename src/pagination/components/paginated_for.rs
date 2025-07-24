use std::{marker::PhantomData, sync::Arc};

use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator};

use crate::{
    cache::CacheStoreFields,
    item_state::ItemState,
    pagination::{use_pagination, PaginationState, UsePaginationOptions},
    InternalLoader, ItemWindow,
};

/// Slot that is rendered when an error occurs.
#[derive(Clone)]
#[slot]
pub struct LoadError {
    children: Arc<dyn Fn(String) -> AnyView + Send + Sync>,
}

/// Slot that is rendered when the data is being loaded.
#[derive(Clone)]
#[slot]
pub struct Loading {
    children: ChildrenFn,
}

/// Quite similar to Leptos' `<For>` this displays a list of items.
///
/// But these items are loaded and cached on-demand using the provided `loader`.
///
/// ## Example
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_windowing::pagination::{Loading, PaginatedFor, PaginationState};
///
/// pub struct Book {
///     title: String,
///     author: String,
/// }
///
/// let state = PaginationState::new_store();
///
/// view! {
///     <ul>
///         <PaginatedFor
///             loader=BookLoader
///             query=()
///             state
///             item_count_per_page=20
///             let:idx_book
///         >
///             // Shown when the data has finished loading.
///             <li class={if idx_book.0 % 2 == 0 { "even" } else { "odd" }}>
///                 <h3>{idx_book.1.title}</h3>
///                 <p>{idx_book.1.author}</p>
///             </li>
///
///             // Shown while the data is loading.
///             <Loading slot>
///                 <li class="loading">Loading...</li>
///             </Loading>
///         </PaginatedFor>
///     </ul>
///
///     <div class="pagination-buttons">
///         <PaginationPrev state attr:class="pagination-prev">
///             "Previous"
///         </PaginationPrev>
///         <PaginationNext state attr:class="pagination-next">
///             "Next"
///         </PaginationNext>
///     </div>
/// }
///
/// pub struct BookLoader;
///
/// impl ExactLoader for BookLoader {
///     type Item = Book;
///     type Query = ();
///     type Error = ();
///
///     async fn load_items(&self, range: Range<usize>, query: &Self::Query) -> Result<Vec<Self::Item>, Self::Error> {
///         todo!()
///     }
///
///     async fn item_count(&self, _query: &Self::Query) -> Result<Option<usize>, Self::Error> {
///         todo!()
///     }
/// }
///
/// ```
///
/// For more in-depth demonstration, please refer to the example `pagination_rest_api`.
#[component]
pub fn PaginatedFor<T, L, Q, CF, V, M>(
    /// The loader to get the data on-demand.
    loader: L,

    /// The query to get the data on-demand.
    #[prop(into)]
    query: Signal<Q>,

    /// The pagination state.
    ///
    /// Used to communicate between the pagination controls and this component.
    state: Store<PaginationState>,

    /// How many items to display per page.
    #[prop(into)]
    item_count_per_page: Signal<usize>,

    /// How many pages to load before and after the current page.
    ///
    /// A value of 1 means that the current page as well as the one before and after will be loaded.
    /// Defaults to 1.
    #[prop(default = 1)]
    overscan_page_count: usize,

    /// Slot that is rendered instead of `children` when the data is being loaded.
    /// This is recommended to be used to show a loading skeleton.
    #[prop(optional)]
    loading: Option<Loading>,

    /// Slot that is rendered instead of `children` when an error occurs.
    #[prop(optional)]
    load_error: Option<LoadError>,

    /// The normal children are rendered when an item is loaded.
    /// This would be a normal `<li>` or `<tr>` element for example.
    children: CF,

    #[prop(optional)] _marker: PhantomData<(M, L)>,
) -> impl IntoView
where
    T: Send + Sync + 'static,
    L: InternalLoader<M, Item = T, Query = Q> + 'static,
    Q: Send + Sync + 'static,
    CF: Fn((usize, Arc<T>)) -> V + Send + Clone + 'static,
    V: IntoView,
{
    let window: ItemWindow<T> = use_pagination(
        state,
        loader,
        query,
        item_count_per_page,
        UsePaginationOptions::default().overscan_page_count(overscan_page_count),
    );

    view! {
        <For each=move || window.range.get() key=|idx| *idx let:index>
            {
                let children = children.clone();
                let loading = loading.clone();
                let load_error = load_error.clone();
                move || match &*window.cache.items().at_unkeyed(index).read() {
                    ItemState::Loaded(item) => {
                        children.clone()((index, Arc::clone(item))).into_any()
                    }
                    ItemState::Error(error) => {
                        load_error
                            .clone()
                            .map(|e| (e.children)(error.clone()).into_any())
                            .unwrap_or_else(|| {

                                view! { <div style="color: red;">Error: {error.clone()}</div> }
                                    .into_any()
                            })
                    }
                    _ => {
                        loading
                            .clone()
                            .map(|l| (l.children)().into_any())
                            .unwrap_or_else(|| ().into_any())
                    }
                }
            }
        </For>
    }
}
