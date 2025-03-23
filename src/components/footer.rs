use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-footer-bg text-footer-text p-4 text-center">
            { "© 2025 rivers-syle systems" }
        </footer>
    }
}
