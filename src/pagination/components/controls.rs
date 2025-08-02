use leptos::prelude::*;
use reactive_stores::Store;

use crate::pagination::{
    PaginationControls, PaginationState, PaginationStateStoreFields, UsePaginationControlsOptions,
    use_pagination_controls,
};

/// A component that renders pagination page controls.
///
/// ## Example
///
/// ```
/// ```
#[component]
pub fn PaginationPages(
    /// The current state of the pagination. This is used to communicate with the PaginatedFor component.
    state: Store<PaginationState>,

    /// How many pages to show around the current page. This number includes the current page.
    ///
    /// A value of 3 will display one page before and one page after the current page.
    /// It's recommended to use odd numbers to ensure symmetry.
    ///
    /// Default is 5.
    #[prop(default = 5)]
    display_page_count: usize,

    /// How many pages to show at the beginning and end of the pagination.
    ///
    /// Default is 1.
    #[prop(default = 1)]
    margin_page_count: usize,

    /// The separator to use between page ranges.
    ///
    /// Default is "…"
    #[prop(into, default = "⋯".into())]
    separator: Signal<String>,

    /// The class of the `<a>` element that represents a page.
    #[prop(into, optional)]
    anchor_class: Signal<String>,

    /// The class of the `<li>` element that wraps the `<a>` element that represents a page.
    #[prop(into, optional)]
    li_class: Signal<String>,

    /// The class of the `<li>` element that represents an active page.
    /// This will be used instead of the `li_class` when the page is active.
    #[prop(into, optional)]
    active_class: Signal<String>,

    /// Every range is put inside an `<ul>` element.
    /// The class of this `<ul>` element can be customized using this prop.
    #[prop(into, optional)]
    ul_class: Signal<String>,

    /// The class of the `<div>` element that contains the separator.
    #[prop(into, optional)]
    separator_class: Signal<String>,
) -> impl IntoView {
    let PaginationControls {
        current_page,
        start_range,
        end_range,
        current_range,
        show_separator_before,
        show_separator_after,
        page_count_error,
    } = use_pagination_controls(
        state,
        UsePaginationControlsOptions::default()
            .display_page_count(display_page_count)
            .margin_page_count(margin_page_count),
    );

    view! {
        {move || {
            page_count_error.get().map(|error| view! { <div class="error-message">{error}</div> })
        }}
        <PaginationRange
            state
            current_page
            range=start_range
            ul_class
            anchor_class
            li_class
            active_class
        />
        <Show when=move || show_separator_before.get()>
            <div class=separator_class>{separator}</div>
        </Show>
        <PaginationRange
            state
            current_page
            range=current_range
            ul_class
            anchor_class
            li_class
            active_class
        />
        <Show when=move || show_separator_after.get()>
            <div class=separator_class>{separator}</div>
        </Show>
        <PaginationRange
            state
            current_page
            range=end_range
            ul_class
            anchor_class
            li_class
            active_class
        />
    }
}

/// Used by `PaginationPages` to render the pagination ranges (button groups).
#[component]
pub fn PaginationRange(
    state: Store<PaginationState>,
    current_page: Signal<usize>,
    range: Signal<Vec<usize>>,
    ul_class: Signal<String>,
    li_class: Signal<String>,
    anchor_class: Signal<String>,
    active_class: Signal<String>,
) -> impl IntoView {
    view! {
        <Show when=move || !range.get().is_empty()>
            <ul class=ul_class>
                <For
                    each=move || range.get()
                    key=|i| *i
                    children=move |index| {
                        let class = Signal::derive(move || {
                            if current_page.get() == index {
                                active_class.get()
                            } else {
                                li_class.get()
                            }
                        });

                        view! {
                            <li class=class>
                                <a
                                    class=anchor_class
                                    on:click=move |evt| {
                                        evt.prevent_default();
                                        state.current_page().set(index);
                                    }
                                >
                                    {index + 1}
                                </a>
                            </li>
                        }
                    }
                />
            </ul>
        </Show>
    }
}

#[component]
/// Button to navigate to the next page.
pub fn PaginationNext(
    /// The current state of the pagination. This is used to communicate with the PaginatedFor component.
    state: Store<PaginationState>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| PaginationState::next(state)
            prop:disabled=move || PaginationState::is_last_page(state)
        >
            {children()}
        </button>
    }
}

#[component]
/// Button to navigate to the previous page.
pub fn PaginationPrev(
    /// The current state of the pagination. This is used to communicate with the PaginatedFor component.
    state: Store<PaginationState>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| PaginationState::prev(state)
            prop:disabled=move || PaginationState::is_first_page(state)
        >
            {children()}
        </button>
    }
}
