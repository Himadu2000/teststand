use leptos::{ReadSignal, Resource};

pub struct Data {
    pub value: ReadSignal<u8>,
    pub text: Resource<(), String>,
}
