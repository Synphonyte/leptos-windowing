use leptos::prelude::*;
use leptos_windowing::ExactLoader;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::QueryBuilder;
use std::ops::Range;

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Customer {
    pub customer_id: String,
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub city: String,
    pub country: String,
    pub phone: String,
    pub email: String,
    pub website: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerServerQuery {
    range: Range<usize>,
    name: String,
}

#[cfg(feature = "ssr")]
fn build_query<'a>(builder: &mut QueryBuilder<'a, sqlx::Sqlite>, name: &'a String)
{
    if !name.is_empty() {
        builder.push("WHERE first_name LIKE concat('%', ");
        builder.push_bind(name);
        builder.push(", '%') OR last_name LIKE concat('%', ");
        builder.push_bind(name);
        builder.push(", '%') OR company LIKE concat('%', ");
        builder.push_bind(name);
        builder.push(", '%') ");
    }
}

#[server]
pub async fn list_customers(query: CustomerServerQuery) -> Result<Vec<Customer>, ServerFnError> {
    use crate::database::get_db;

    let CustomerServerQuery {  range, name } = query;

    let mut builder = QueryBuilder::new("SELECT customer_id, first_name, last_name, company, city, country, phone, email, website FROM customers ");
    build_query(&mut builder, &name);

    builder.push(" LIMIT ");
    builder.push_bind(range.len() as i64);
    builder.push(" OFFSET ");
    builder.push_bind(range.start as i64);

    builder
        .build_query_as::<Customer>()
        .fetch_all(get_db())
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))
}

#[server]
pub async fn customer_count(name: String) -> Result<usize, ServerFnError> {
    use crate::database::get_db;

    let mut builder = QueryBuilder::new("SELECT COUNT(*) FROM customers ");

    build_query(&mut builder, &name);

    let count: (i64,) = builder
        .build_query_as::<(i64,)>()
        .fetch_one(get_db())
        .await
        .map_err(|err| ServerFnError::new(format!("{err:?}")))?;


    Ok(count.0 as usize)
}

#[derive(Debug, Default)]
pub struct CustomerQuery {
    pub name: String,
}

pub struct CustomerLoader;

impl ExactLoader for CustomerLoader {
    type Item = Customer;
    type Query = CustomerQuery;
    type Error = ServerFnError;

    async fn load_items(&self, range: Range<usize>, query: &Self::Query) -> Result<Vec<Customer>, Self::Error> {
        list_customers(CustomerServerQuery {
            name: query.name.clone(),
            range: range.clone(),
        })
        .await
    }

    async fn item_count(&self, query: &Self::Query) -> Result<Option<usize>, Self::Error> {
        customer_count(query.name.clone()).await.map(Some)
    }
}
