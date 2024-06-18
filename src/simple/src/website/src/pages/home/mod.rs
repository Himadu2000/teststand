mod data;
mod view;

use cynic::{http::SurfExt, QueryBuilder};
use data::Data;
use leptos::*;
use view::View;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal::<u8>(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    let add = move |_| set_value.update(|value| *value += 1);
    let sub = move |_| set_value.update(|value| *value -= 1);

    let data = Data { value };

    view! {
        <View data=data events=(add, sub) />
    }
}
