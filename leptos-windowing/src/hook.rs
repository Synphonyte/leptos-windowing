use std::sync::Arc;

use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos_use::core::IntoElementMaybeSignal;

use crate::{item_state::ItemState, InternalLoader};

pub fn use_windowing<T: Send + Sync + 'static, El, ElM, LoaderM>(
    loader: impl InternalLoader<LoaderM, Item = T>,
    scroll_element: impl IntoElementMaybeSignal<web_sys::Element, ElM>,
    estimate_item_size: impl Fn(usize) -> f32 + Send + Sync + 'static,
    options: UseWindowingOptions,
) -> UseWindowingReturn<T> {
    todo!()
}

/// Return type of [`use_windowing`].
pub struct UseWindowingReturn<T: Send + Sync + 'static> {
    /// The number of items before the window, i.e. before the first item in [`items`].
    pub item_count_before: Signal<usize>,

    /// The number of items after the window, i.e. after the last item in [`items`].
    pub item_count_after: Signal<usize>,

    /// A list of signals for every item in the window.
    ///
    /// When the window position changes, the signals are updated.
    /// Only if the window size changes, the length of this `Vec` is updated together with
    /// [`window_size`].
    pub items: Vec<Signal<ItemState<T>>>,

    /// This is a signal for the length of [`items`].
    pub window_size: Signal<usize>,
}

#[derive(DefaultBuilder)]
pub struct UseWindowingOptions {
    measure_item: Arc<dyn Fn(usize) -> f64 + Send + Sync>,
}
