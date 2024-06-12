mod view;

use leptos::*;
use leptos_meta::*;
use view::View;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <View />
    }
}
