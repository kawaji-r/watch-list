use yew::prelude::*;

pub struct SearchBar;

impl Component for SearchBar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SearchBar
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="search-bar">
                <input type="text" placeholder="番組を検索..." />
            </div>
        }
    }
}
