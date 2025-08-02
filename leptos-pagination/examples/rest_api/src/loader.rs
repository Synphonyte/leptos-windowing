use std::fmt::Display;

use gloo_net::http::{Request, RequestBuilder};
use leptos_pagination::{PaginatedCount, PaginatedLoader};

use crate::models::{Brewery, MetaResponse};


pub struct BreweryLoader;

#[derive(Debug, Clone, Copy, Default)]
pub enum SortDirection {
    Ascending,
    Descending,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Column {
    #[default]
    Name,
    City,
    Country,
}

#[derive(Default, Clone)]
pub struct BreweryQuery {
    pub sorting_column: Column,
    pub sorting_direction: SortDirection,
}

impl BreweryLoader {
    fn url_sort_param_for_sort_pair(&self, pair: &(Column, SortDirection)) -> (&'static str, String) {
        let dir = pair.1.to_api();

        ("sort", format!("{}:{}", pair.0.to_string().to_lowercase(), dir))
    }

    fn get_builder(&self, page_index: usize, query: &BreweryQuery) -> RequestBuilder {
        let mut query_pairs = vec![
            ("page", (page_index + 1).to_string()),
            ("per_page", Self::PAGE_ITEM_COUNT.to_string()),
        ];

        if !matches!(query.sorting_direction, SortDirection::None) {
            query_pairs.push(self.url_sort_param_for_sort_pair(&(query.sorting_column, query.sorting_direction)));
        }

        Request::get("https://api.openbrewerydb.org/v1/breweries").query(query_pairs)
    }
}

impl PaginatedLoader for BreweryLoader {
    const PAGE_ITEM_COUNT: usize = 20;

    type Item = Brewery;

    type Query = BreweryQuery;

    type Error = anyhow::Error;

    async fn load_page(
        &self,
        page_index: usize,
        query: &Self::Query,
    ) -> Result<Vec<Self::Item>, Self::Error> {
        if page_index >= 10000 / Self::PAGE_ITEM_COUNT {
            return Ok(vec![]);
        }

        let resp: Vec<Brewery> = self.get_builder(page_index, query)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    async fn count(&self, _query: &Self::Query) -> Result<Option<PaginatedCount>, Self::Error> {
        let resp: MetaResponse = Request::get("https://api.openbrewerydb.org/v1/breweries/meta")
            .send()
            .await?
            .json()
            .await?;

        Ok(Some(PaginatedCount::Items(resp.total)))
    }
}

impl SortDirection {
    fn to_api(&self) -> &'static str {
        match self {
            SortDirection::Ascending => "asc",
            SortDirection::Descending => "desc",
            SortDirection::None => "",
        }
    }

    pub fn next(&mut self) {
        match self {
            SortDirection::Ascending => *self = SortDirection::Descending,
            SortDirection::Descending => *self = SortDirection::None,
            SortDirection::None => *self = SortDirection::Ascending,
        }
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Column::Name => write!(f, "Name"),
            Column::City => write!(f, "City"),
            Column::Country => write!(f, "Country"),
        }
    }
}
