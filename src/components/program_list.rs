use crate::components::program_card::{ProgramCard, ProgramCardProps};
use yew::prelude::*;

pub struct ProgramList;

impl Component for ProgramList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ProgramList
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // サンプルデータ
        let programs = vec![
            ProgramCardProps {
                thumbnail: "https://via.placeholder.com/300x180".to_string(),
                title: "サンプル番組 A".to_string(),
                description: "この番組はおしゃれで面白いコンテンツを提供します。".to_string(),
                broadcast_date: "2025/03/15".to_string(),
                episode: "第1回目".to_string(),
                watched: false,
            },
            ProgramCardProps {
                thumbnail: "https://via.placeholder.com/300x180".to_string(),
                title: "サンプル番組 B".to_string(),
                description: "毎週楽しい放送があなたを待っています。".to_string(),
                broadcast_date: "2025/03/20".to_string(),
                episode: "第2回目".to_string(),
                watched: true,
            },
        ];

        html! {
            <div class="program-list">
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
}
