use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgramCardProps {
    pub thumbnail: Option<String>,
    pub title: String,
    pub description: String,
    pub broadcast_date: String,
    pub episode: u16,
    pub watched: bool,
}

#[function_component(ProgramCard)]
pub fn program_card(props: &ProgramCardProps) -> Html {
    let status_class = if props.watched {
        "bg-green-200 text-green-800"
    } else {
        "bg-red-200 text-red-800"
    };
    let status_text = if props.watched {
        "視聴済み"
    } else {
        "未視聴"
    };

    html! {
        <div class="flex flex-col w-80 bg-white shadow rounded-lg overflow-hidden transition-transform hover:-translate-y-1 cursor-pointer">
            {if let Some(thumbnail) = &props.thumbnail {
                html! {<img class="w-full h-40 object-cover" src={thumbnail.clone()} alt={props.title.clone()} />}
            } else {
                html! {<div class="text-3xl text-bold text-white w-full h-40 bg-gray-300 flex items-center justify-center">{"No Image"}</div>}
            }}
            <div class="p-4 flex flex-col flex-1">
                <div class="text-lg font-semibold mb-2">{ props.title.clone() + " # " + &props.episode.to_string() }</div>
                <div class="flex-1 min-h-0 text-gray-700 text-sm mb-4">{ props.description.clone() }</div>
                <div class="p-2 rounded mb-4">
                    <p class="text-sm">{ format!("放送日: {}", props.broadcast_date) }</p>
                </div>
                <div class="flex items-center justify-between">
                    <span class={classes!("px-2", "py-1", "rounded", status_class)}>{ status_text }</span>
                    <button class="bg-white border-primary border-2 text-primary px-3 py-1 rounded hover:bg-primary-dark">{ "更新" }</button>
                </div>
            </div>
        </div>
    }
}
