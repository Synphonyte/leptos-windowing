use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos_use::math::{use_not, use_or};
use reactive_stores::Store;

use crate::pagination::{PaginationState, PaginationStateStoreFields};

pub fn use_pagination_controls(
    state: Store<PaginationState>,
    options: UsePaginationControlsOptions,
) -> PaginationControls {
    let UsePaginationControlsOptions {
        display_page_count,
        margin_page_count,
    } = options;

    let page_count = Signal::derive(move || state.page_count().get().unwrap_or_default());

    let additional_page_count = display_page_count / 2;

    let current_page: Signal<usize> = state.current_page().into();
    let current_range_start =
        Signal::derive(move || current_page.get().saturating_sub(additional_page_count));
    let current_range_end =
        Signal::derive(move || current_page.get().saturating_add(additional_page_count));

    let merge_current_with_start =
        Memo::new(move |_| current_range_start.get() <= margin_page_count);
    let merge_current_with_end = Memo::new(move |_| {
        current_range_end.get() + 1 >= page_count.get().saturating_sub(margin_page_count)
    });

    let start_range_end = Signal::derive(move || {
        if merge_current_with_start.get() {
            current_range_end.get()
        } else {
            margin_page_count + 1
        }
    });

    let end_range_start = Signal::derive(move || {
        if merge_current_with_end.get() {
            current_range_start.get() + 1
        } else {
            page_count.get().saturating_sub(margin_page_count + 1)
        }
    });

    let merge_all = Signal::derive(move || start_range_end.get() + 1 >= end_range_start.get());

    PaginationControls {
        current_page,
        start_range: Memo::new(move |_| {
            let end = if merge_all.get() {
                page_count.get()
            } else {
                start_range_end.get()
            };

            (0..end).collect()
        })
        .into(),
        end_range: Memo::new(move |_| {
            if merge_all.get() {
                vec![]
            } else {
                let start = end_range_start.get();
                let end = page_count.get();
                (start..end).collect()
            }
        })
        .into(),
        current_range: Memo::new(move |_| {
            if merge_current_with_start.get() || merge_current_with_end.get() || merge_all.get() {
                vec![]
            } else {
                let start = current_page.get() - margin_page_count;
                let end = current_page.get() + margin_page_count;
                (start..=end).collect()
            }
        })
        .into(),
        show_separator_before: use_not(use_or(merge_current_with_start, merge_all)),
        show_separator_after: use_not(use_or(merge_current_with_end, merge_all)),
        page_count_error: state.page_count_error().into(),
    }
}

/// Return type of [`use_pagination_controls`]. It provides a bunch of signals to easily build a pagination component.
///
/// Please note that all ranges are inclusive. This means that the start and end of each range are included in the range.
/// Also counting starts from 0. The first page is 0.
#[derive(Debug, Copy, Clone)]
pub struct PaginationControls {
    /// If the page count couldn't be determined, this signal will contain an error message.
    pub page_count_error: Signal<Option<String>>,

    pub current_page: Signal<usize>,

    /// The range of pages at the start of the pagination.
    ///
    /// In many cases this will be `0` to `margin_page_count`.
    ///
    /// But if the current range is too close or overlaps then this start range with be extended so it includes the current range.
    /// The current range will then be returned as empty.
    ///
    /// If there are so few pages that all the ranges (start, end, current) need to be merged into one range then they will all
    /// be merged into this start range and they will be returned emtpy.
    pub start_range: Signal<Vec<usize>>,

    /// The range of pages at the end of the pagination.
    ///
    /// In many cases this will be `total_pages - margin_page_count` to `total_pages`.
    ///
    /// But if the current range is too close or overlaps then this end range with be extended so it includes the current range.
    /// The current range will then be returned as empty.
    ///
    /// If there are so few pages that all the ranges (start, end, current) need to be merged into one range then they will all
    /// be merged into the start range and this will be empty.
    pub end_range: Signal<Vec<usize>>,

    /// The current range of pages. This will be empty if the current range is too close to the start or the end. In this case
    /// the range will be merged with the start or end range.
    pub current_range: Signal<Vec<usize>>,

    /// Whether to show a separator (usually an ellipsis "...") before the current range.
    pub show_separator_before: Signal<bool>,

    /// Whether to show a separator (usually an ellipsis "...") after the current range.
    pub show_separator_after: Signal<bool>,
}

/// Options for [`use_pagination`].
#[derive(Debug, Clone, DefaultBuilder)]
pub struct UsePaginationControlsOptions {
    /// How many pages to show around the current page. This number includes the current page.
    ///
    /// A value of 3 will display one page before and one page after the current page.
    /// It's recommended to use odd numbers to ensure symmetry.
    ///
    /// Default is 5.
    display_page_count: usize,

    /// How many pages to show at the beginning and end of the pagination.
    ///
    /// Default is 1.
    margin_page_count: usize,
}

impl Default for UsePaginationControlsOptions {
    fn default() -> Self {
        Self {
            display_page_count: 5,
            margin_page_count: 1,
        }
    }
}
