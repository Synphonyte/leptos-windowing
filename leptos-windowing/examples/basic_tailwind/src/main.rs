use std::ops::Range;

use leptos::prelude::*;
use leptos_windowing::{
    pagination::{
        Loading, PaginatedFor, PaginationNext, PaginationPages, PaginationPrev, PaginationState,
    },
    MemoryLoader, SortMode,
};
use leptos_windowing_examples::data::{Book, BOOKS};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    let state = PaginationState::new();

    view! {
        <ul class="m-10 text-sm rounded-md border dark:border-gray-700 overflow-clip">
            <PaginatedFor loader=BookLoader state item_count_per_page=5 let:book>
                <li class="p-2 border-b dark:border-gray-700">
                    <h3 class="font-bold">{book.1.title}</h3>
                    <p class="text-gray-500 dark:text-gray-400">{book.1.author}</p>
                </li>

                <Loading slot>
                    // Skeleton loading
                    <li class="p-2 border-b animate-pulse dark:border-gray-700">
                        <div>
                            <div class="my-1 w-3/4 h-4 bg-gray-300 rounded dark:bg-gray-600"></div>
                            <div class="my-1 w-1/2 h-3 bg-gray-200 rounded dark:bg-gray-700"></div>
                        </div>
                    </li>
                </Loading>
            </PaginatedFor>
        </ul>

        <div class="flex justify-between">
            <nav aria-label="Page navigation" class="flex justify-start m-10">
                <PaginationPrev
                    state
                    attr:class="px-4 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-l-lg hover:bg-gray-100 hover:text-gray-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white dark:focus:ring-blue-500 dark:focus:text-white cursor-pointer select-none"
                >
                    Prev
                </PaginationPrev>
                <PaginationNext
                    state
                    attr:class="px-4 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-r-lg hover:bg-gray-100 hover:text-gray-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white dark:focus:ring-blue-500 dark:focus:text-white border-l-0 cursor-pointer select-none"
                >
                    Next
                </PaginationNext>
            </nav>

            <nav aria-label="Page number navigation" class="flex justify-end m-10">
                <PaginationPages
                    state
                    ul_class="inline-flex text-sm"
                    li_class="border border-gray-300 dark:border-gray-700 border-l-0 first:border-l first:rounded-l-lg last:rounded-r-lg bg-white dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700"
                    active_class="bg-blue-600 font-bold hover:bg-blue-700 dark:bg-blue-600 dark:hover:bg-blue-700 first:rounded-l-lg last:rounded-r-lg *:text-white *:hover:text-white dark:*:text-white dark:*:hover:text-white"
                    anchor_class="flex items-center justify-center px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-white cursor-pointer select-none"
                    separator_class="self-center select-none mx-2"
                />
            </nav>
        </div>
    }
}

pub struct BookLoader;

impl MemoryLoader for BookLoader {
    type Item = Book;

    // We're going to ignore sorting for this simple example.
    fn load_items(&self, range: Range<usize>, _sorting: &[(usize, SortMode)]) -> Vec<Self::Item> {
        BOOKS[range.clone()].to_vec()
    }

    fn item_count(&self) -> usize {
        BOOKS.len()
    }
}
