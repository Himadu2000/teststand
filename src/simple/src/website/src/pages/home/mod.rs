mod view;

use leptos::*;
use view::View;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout

    fn add() {
        set_value.update(|value| *value += 1)
    }

    fn sub() {
        set_value.update(|value| *value -= 1)
    }

    view! {
        <View add sub />
    }
}
