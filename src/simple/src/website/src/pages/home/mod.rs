mod data;
mod query;
mod view;

use crate::pages::client::client;
use cynic::{http::ReqwestExt, GraphQlResponse, QueryBuilder};
use data::Data;
use leptos::*;
use query::UnnamedQuery;
use reqwest::Client;
use view::View;

#[island]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal::<u8>(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    let operation = UnnamedQuery::build(());

    let response = create_resource(
        || (),
        |_| async move {
            client::<UnnamedQuery>(UnnamedQuery::build(()))
                .await
                .unwrap()
                .status
        },
    );

    let add = move |_| set_value.update(|value| *value += 1);
    let sub = move |_| set_value.update(|value| *value -= 1);

    let data = Data {
        value,
        text: response,
    };

    view! {
        <View data=data events=(add, sub) />
    }
}
