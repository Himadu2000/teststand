mod data;
mod query;
mod view;

use cynic::{http::ReqwestExt, QueryBuilder};
use data::Data;
use leptos::*;
use query::UnnamedQuery;
use view::View;
use reqwest::Client;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal::<u8>(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    let operation = UnnamedQuery::build(());

    let response = Client::new()
    .post("http://127.0.0.1:8000/graphql")
    .json(&operation)
    .send()
    .unwrap();

let all_films_result = response.json::<GraphQlResponse<UnnamedQuery>>.unwrap();

    let add = move |_| set_value.update(|value| *value += 1);
    let sub = move |_| set_value.update(|value| *value -= 1);

    let data = Data { value };

    view! {
        <View data=data events=(add, sub) />
    }
}
