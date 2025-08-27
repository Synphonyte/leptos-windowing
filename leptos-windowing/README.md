# Leptos Windowing

[![Crates.io](https://img.shields.io/crates/v/leptos-windowing.svg)](https://crates.io/crates/leptos-windowing)
[![Docs](https://docs.rs/leptos-windowing/badge.svg)](https://docs.rs/leptos-windowing/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/synphonyte/leptos-windowing#license)
[![Build Status](https://github.com/synphonyte/leptos-windowing/actions/workflows/cd.yml/badge.svg)](https://github.com/synphonyte/leptos-windowing/actions/workflows/cd.yml)

<!-- cargo-rdme start -->

Base crate for virtualization and pagination for Leptos.

This crate contains common code for the crates leptos-pagination and leptos-virtualization.
You probably want don't want to use this crate directly.

### Loading data

Wether you use pagination or virtualization you have to provide the data to display.
This is done through implementing one of the various `Loader` traits. Depending on your use case
you should implement the trait that best fits your needs:

- [`MemoryLoader`]: If your dataset is already in memory like in a `Vec`, `HashSet`, array, ...
- [`PaginatedLoader`]: If your data source provides data in pages (independent of if you use UI pagination or virtualization).
- [`ExactLoader`]: If your data source can provide an exact range of items (start index to end index).
- [`Loader`]: If none of the above fit your needs, you can implement this trait to provide your own loading logic.

Please refer to the documentation and the examples to see how to implement these traits.

<!-- cargo-rdme end -->
