use leptos::prelude::*;
use leptos_meta::*;
use leptos_pagination::*;
use leptos_router::components::{FlatRoutes, Route, Router, RoutingProgress};
use leptos_router::path;

use std::sync::Arc;
use std::time::Duration;

use crate::loader::{Customer, CustomerLoader, CustomerQuery};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/serverfn-sqlx.css" />
        <Title text="Welcome to Leptos Struct Table" />

        <Router set_is_routing>
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=Duration::from_millis(250) />
            </div>
            <main>
                <FlatRoutes fallback=|| "Not Found">
                    <Route path=path!("") view=HomePage />
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let state = PaginationState::new_store();

    let query = RwSignal::new(CustomerQuery::default());

    view! {
        <div class="flex flex-col bg-white h-[100vh]">
            <div class="py-2 px-5 border-b border-gray-300 bg-slate-100">
                <label class="block relative">
                    <span class="flex absolute inset-y-0 left-0 items-center pl-3">
                        <svg
                            class="w-5 h-5 fill-black"
                            xmlns="http://www.w3.org/2000/svg"
                            x="0px"
                            y="0px"
                            width="30"
                            height="30"
                            viewBox="0 0 30 30"
                        >
                            <path d="M 13 3 C 7.4889971 3 3 7.4889971 3 13 C 3 18.511003 7.4889971 23 13 23 C 15.396508 23 17.597385 22.148986 19.322266 20.736328 L 25.292969 26.707031 A 1.0001 1.0001 0 1 0 26.707031 25.292969 L 20.736328 19.322266 C 22.148986 17.597385 23 15.396508 23 13 C 23 7.4889971 18.511003 3 13 3 z M 13 5 C 17.430123 5 21 8.5698774 21 13 C 21 17.430123 17.430123 21 13 21 C 8.5698774 21 5 17.430123 5 13 C 5 8.5698774 8.5698774 5 13 5 z"></path>
                        </svg>
                    </span>
                    <input
                        class="py-2 pr-4 pl-10 w-full bg-white rounded-full border focus:outline-none placeholder:font-italitc border-slate-300"
                        placeholder="Search by name or company"
                        type="text"
                        value=move || query.read().name.clone()
                        on:change=move |e| query.write().name = event_target_value(&e)
                    />
                </label>
            </div>

            <div class="grid grid-cols-4 gap-4 p-5 min-h-0 text-sm text-left text-gray-500 dark:text-gray-400 grow">
                <PaginatedFor
                    loader=CustomerLoader
                    state
                    query
                    item_count_per_page=12
                    let:customer_item
                >
                    <CustomerCard customer_item />

                    <Loading slot>
                        // Skeleton loading
                        <CustomerCardSkeleton />
                    </Loading>
                </PaginatedFor>
            </div>
        </div>

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
pub fn CustomerCardSkeleton() -> impl IntoView {
    view! {
        <article class="overflow-hidden w-full max-w-sm bg-white rounded-lg shadow-lg dark:bg-gray-700">
            <div class="flex flex-col gap-1 px-4 mt-4 mb-4">
                // Name skeleton - larger bar for full name
                <div class="w-3/4 h-6 bg-gray-300 rounded animate-pulse"></div>

                // Company skeleton - medium bar for company name
                <div class="w-2/3 h-5 bg-gray-300 rounded animate-pulse"></div>

                // Location skeleton - smaller bar for city and country
                <div class="w-1/2 h-4 bg-gray-300 rounded animate-pulse"></div>
            </div>
        </article>
    }
}

#[component]
pub fn CustomerCard(customer_item: WindowItem<Customer>) -> impl IntoView {
    let customer = Arc::clone(&customer_item.data);

    let handle_delete = move |_| {
        customer_item.remove();
    };

    view! {
        <article class="relative overflow-hidden w-full max-w-sm min-h-0 bg-white rounded-lg shadow-lg dark:bg-gray-700">
            <DeleteButton on:click=handle_delete />

            <div class="flex flex-col gap-1 px-4 mt-4 mb-4">
                <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-50">
                    {customer.first_name.clone()} " " {customer.last_name.clone()}
                </h2>
                <a
                    class="font-semibold text-gray-800 dark:text-gray-50"
                    href=customer.website.clone()
                >
                    {customer.company.clone()}
                </a>
                <span class="font-normal text-gray-600 dark:text-gray-300">
                    {customer.city.clone()} " in " {customer.country.clone()}
                </span>
            </div>
        </article>
    }
}

#[component]
pub fn DeleteButton() -> impl IntoView {
    view! {
        <button
            class="p-2 text-grey-500 opacity-50 hover:opacity-100 absolute top-0 right-0"
            title="Delete customer"
        >
            <svg
                class="block size-6"
                fill="currentColor"
                viewBox="0 0 20 20"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    fill-rule="evenodd"
                    d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                    clip-rule="evenodd"
                ></path>
            </svg>
        </button>
    }
}
