use yew::prelude::*;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Header
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header>
                <div class="logo">{ "ウォッチリスト！" }</div>
                <nav>
                    <a href="#list">{ "ウォッチリスト一覧" }</a>
                    <a href="#register">{ "番組登録" }</a>
                    <a href="#about">{ "サイト説明" }</a>
                    <a href="#login">{ "ログイン" }</a>
                </nav>
            </header>
        }
    }
}
