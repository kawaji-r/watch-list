use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer>
                { "© 2025 ウォッチリスト！ All Rights Reserved." }
            </footer>
        }
    }
}
