use std::sync::Arc;

pub enum ItemState<T: Send + Sync + 'static> {
    /// The row is not yet loaded and a placeholder is displayed if the row is visible in the viewport.
    Placeholder,
    /// The row is loading and a placeholder is displayed if the row is visible in the viewport.
    Loading,
    /// The row has been loaded.
    Loaded(Arc<T>),
    /// The row failed to load.
    Error(String),
}

impl<T: Send + Sync + 'static> Clone for ItemState<T> {
    fn clone(&self) -> Self {
        match self {
            ItemState::Placeholder => ItemState::Placeholder,
            ItemState::Loading => ItemState::Loading,
            ItemState::Loaded(item) => ItemState::Loaded(Arc::clone(item)),
            ItemState::Error(error) => ItemState::Error(error.clone()),
        }
    }
}

impl<T: Send + Sync + 'static> std::fmt::Debug for ItemState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemState::Placeholder => write!(f, "Placeholder"),
            ItemState::Loading => write!(f, "Loading"),
            ItemState::Loaded(_) => write!(f, "Loaded"),
            ItemState::Error(e) => write!(f, "Error({e})"),
        }
    }
}
