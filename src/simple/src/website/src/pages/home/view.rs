use super::data::Data;
use leptos::{ev::MouseEvent, *};
use leptos_meta::*;

#[component]
pub fn View<ME, MB>(data: Data, events: (ME, MB)) -> impl IntoView
where
    ME: Fn(MouseEvent) + 'static,
    MB: Fn(MouseEvent) + 'static,
{
    view! {
        <Title text="Home"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=events.0 class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {data.value}
                    </button>
                    <button on:click=events.1 class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "-"
                    </button>
                    {move || match data.text.get() {
                        None => view! { <p>"Loading..."</p> }.into_view(),
                        Some(data) => view! { <p>{data}</p> }.into_view()
                    }}
                </div>
            </div>
        </main>
    }
}
