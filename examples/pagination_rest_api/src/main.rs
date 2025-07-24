mod loader;
mod models;

use leptos::{either::EitherOf3, prelude::*};
use leptos_windowing::pagination::{
    Loading, PaginatedFor, PaginationNext, PaginationPages, PaginationPrev, PaginationState,
};
use loader::{BreweryLoader, BreweryQuery, Column, SortDirection};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    let state = PaginationState::new_store();

    let query = RwSignal::new(BreweryQuery::default());

    view! {
        <div class="m-10 mb-0 text-sm rounded-md shadow-xs" role="group">
            <div class="mb-2 ml-2 text-gray-900 dark:text-white">"Sort by"</div>
            <div class="flex" role="group">
                <SortButton query column=Column::Name />
                <SortButton query column=Column::City />
                <SortButton query column=Column::Country />
            </div>
        </div>

        <ul class="m-10 text-sm bg-white rounded-md border border-gray-200 dark:bg-gray-800 dark:border-gray-700 overflow-clip">
            <PaginatedFor loader=BreweryLoader query state item_count_per_page=5 let:idx_brewery>
                <li class="p-2 border-b border-gray-200 dark:border-gray-700">
                    <h3 class="font-bold text-gray-900 dark:text-white">
                        {idx_brewery.1.name.clone()}
                    </h3>
                    <p class="text-gray-500 dark:text-gray-400">
                        {idx_brewery.1.city.clone()} ", " {idx_brewery.1.country.clone()}
                    </p>
                    <p class="text-gray-500 dark:text-gray-400">
                        "Website: "
                        {idx_brewery
                            .1
                            .website_url
                            .as_ref()
                            .map(|url| {
                                view! {
                                    <a
                                        class="text-blue-600 dark:text-blue-400"
                                        href=url.clone()
                                        target="_blank"
                                    >
                                        {url.clone()}

                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="inline-block ml-1 align-middle -mt-[2px] size-3"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25"
                                            />
                                        </svg>
                                    </a>
                                }
                                    .into_any()
                            })
                            .unwrap_or(
                                view! {
                                    <span class="text-gray-500 dark:text-gray-400">"N/A"</span>
                                }
                                    .into_any(),
                            )}

                    </p>
                </li>

                <Loading slot>
                    // Skeleton loading
                    <li class="p-2 border-b animate-pulse dark:border-gray-700">
                        <div>
                            <div class="my-1 w-3/4 h-4 bg-gray-300 rounded dark:bg-gray-600"></div>
                            <div class="my-1 w-2/3 h-3.5 bg-gray-200 rounded dark:bg-gray-700"></div>
                            <div class="my-1 w-1/2 h-3.5 bg-gray-200 rounded dark:bg-gray-700"></div>
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
                    separator_class="self-center select-none mx-2 text-gray-500 dark:text-gray-400"
                />
            </nav>
        </div>
    }
}

#[component]
pub fn SortButton(query: RwSignal<BreweryQuery>, column: Column) -> impl IntoView {
    let sort_dir = Signal::derive(move || {
        let query = query.read();

        if query.sorting_column == column {
            query.sorting_direction
        } else {
            SortDirection::None
        }
    });

    view! {
        <button
            type="button"
            class="flex py-2 px-4 font-medium text-gray-900 align-middle whitespace-nowrap bg-white border-t border-b border-gray-200 dark:text-white dark:bg-gray-800 dark:border-gray-700 first:border last:border hover:text-blue-700 hover:bg-gray-100 focus:z-10 focus:text-blue-700 focus:ring-2 focus:ring-blue-700 break dark:hover:text-white dark:hover:bg-gray-700 dark:focus:ring-blue-500 dark:focus:text-white first:rounded-s-lg last:rounded-e-lg"
            on:click=move |_| {
                let mut query = query.write();
                if query.sorting_column == column {
                    query.sorting_direction.next();
                } else {
                    query.sorting_column = column;
                    query.sorting_direction = SortDirection::Ascending;
                }
            }
        >
            {column.to_string()}

            <SortDirectionIcon dir=sort_dir />
        </button>
    }
}

#[component]
pub fn SortDirectionIcon(dir: Signal<SortDirection>) -> impl IntoView {
    move || match dir.get() {
        SortDirection::Ascending => EitherOf3::A(view! {
            <svg
                class="ml-2 text-gray-800 dark:text-white"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 6v13m0-13 4 4m-4-4-4 4"
                />
            </svg>
        }),
        SortDirection::Descending => EitherOf3::B(view! {
            <svg
                class="ml-2 text-gray-800 dark:text-white"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 19V5m0 14-4-4m4 4 4-4"
                />
            </svg>
        }),
        SortDirection::None => EitherOf3::C(()),
    }
}
