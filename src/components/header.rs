use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-primary text-white text-center p-header-padding">
            <div class="text-2xl font-bold p-5">{ "ウォッチリスト！" }</div>
            <nav class="space-x-4">
                <a class="hover:text-secondary" href="#list">{ "ウォッチリスト一覧" }</a>
                <a class="hover:text-secondary" href="#register">{ "番組登録" }</a>
                <a class="hover:text-secondary" href="#about">{ "サイト説明" }</a>
                <a class="hover:text-secondary" href="#login">{ "ログイン" }</a>
            </nav>
        </header>
    }
}
