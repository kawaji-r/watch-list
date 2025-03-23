use crate::components::program_card::{ProgramCard, ProgramCardProps};
use yew::prelude::*;

#[function_component(ProgramList)]
pub fn program_list() -> Html {
    // サンプルデータ
    let programs = vec![
        ProgramCardProps {
            thumbnail: Some("https://placehold.jp/150x150.png".to_string()),
            title: "サンプル番組 A".to_string(),
            description: "この番組はおしゃれで面白いコンテンツを提供します。".to_string(),
            broadcast_date: "2025/03/15".to_string(),
            episode: 1,
            watched: false,
        },
        ProgramCardProps {
            thumbnail: None,
            title: "サンプル番組 B".to_string(),
            description: "毎週楽しい放送があなたを待っています。".to_string(),
            broadcast_date: "2025/03/20".to_string(),
            episode: 2,
            watched: true,
        },
    ];

    html! {
        <div class="flex flex-wrap gap-5 justify-center">
            { for programs.into_iter().map(|program| html! {
                <ProgramCard
                    thumbnail={program.thumbnail}
                    title={program.title}
                    description={program.description}
                    broadcast_date={program.broadcast_date}
                    episode={program.episode}
                    watched={program.watched}
                />
            }) }
        </div>
    }
}
