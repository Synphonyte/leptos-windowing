use gloo_net::http::{Request, RequestBuilder};
use leptos_windowing::{PaginatedCount, PaginatedLoader};

use crate::models::{Brewery, MetaResponse};


pub struct BreweryLoader;

pub enum Sort {
    Ascending,
    Descending,
    None,
}

#[derive(Default)]
pub struct BreweryQuery {
    search: String,
    sorting: Vec<(usize, Sort)>,
}

impl BreweryLoader {
    fn url_sort_param_for_column(&self, column: usize) -> &'static str {
        match column {
            0 => "name",
            1 => "city",
            2 => "country",
            _ => "",
        }
    }

    fn url_sort_param_for_sort_pair(&self, pair: &(usize, Sort)) -> (&'static str, String) {
        let col = self.url_sort_param_for_column(pair.0);
        let dir = pair.1.to_api();

        ("sort", format!("{}:{}", col, dir))
    }

    fn get_builder(&self, page_index: usize, query: &BreweryQuery) -> RequestBuilder {
        let mut query_pairs = vec![
            ("page", (page_index + 1).to_string()),
            ("per_page", Self::PAGE_ITEM_COUNT.to_string()),
        ];

        if !query.search.is_empty() {
            query_pairs.push(("by_name", query.search.clone()));
        }

        for pair in &query.sorting {
            query_pairs.push(self.url_sort_param_for_sort_pair(pair));
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

    async fn count(&self, query: &Self::Query) -> Result<Option<PaginatedCount>, Self::Error> {
        let mut builder = Request::get("https://api.openbrewerydb.org/v1/breweries/meta");

        if !query.search.is_empty() {
            builder = builder.query([("by_name", &query.search)]);
        }

        let resp: MetaResponse = builder
            .send()
            .await?
            .json()
            .await?;

        Ok(Some(PaginatedCount::Items(resp.total)))
    }
}

impl Sort {
    fn to_api(&self) -> &'static str {
        match self {
            Sort::Ascending => "asc",
            Sort::Descending => "desc",
            Sort::None => "",
        }
    }
}
