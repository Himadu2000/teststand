mod data;
mod query;
mod view;

use cynic::{http::ReqwestExt, GraphQlResponse, QueryBuilder};
use data::Data;
use leptos::*;
use query::UnnamedQuery;
use reqwest::Client;
use view::View;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal::<u8>(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    let operation = UnnamedQuery::build(());

    let response = create_resource(
        || (),
        |_| async move {
            Client::new()
                .post("http://127.0.0.1:8000/graphql")
                .json(&operation)
                .send()
                .await
                .unwrap()
                .json::<GraphQlResponse<UnnamedQuery>>()
                .await
                .unwrap()
        },
    );

    let add = move |_| set_value.update(|value| *value += 1);
    let sub = move |_| set_value.update(|value| *value -= 1);

    let data = Data { value };

    view! {
        <View data=data events=(add, sub) />
    }
}
