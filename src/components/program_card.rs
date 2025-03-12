use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgramCardProps {
    pub thumbnail: String,
    pub title: String,
    pub description: String,
    pub broadcast_date: String,
    pub episode: String,
    pub watched: bool,
}

pub struct ProgramCard;

impl Component for ProgramCard {
    type Message = ();
    type Properties = ProgramCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ProgramCard
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let status_class = if ctx.props().watched {
            "watched"
        } else {
            "not-watched"
        };
        let status_text = if ctx.props().watched {
            "視聴済み"
        } else {
            "未視聴"
        };

        html! {
            <div class="program-card">
                <img class="program-thumbnail" src={ctx.props().thumbnail.clone()} alt="サムネイル" />
                <div class="program-content">
                    <div class="program-title">{ &ctx.props().title }</div>
                    <div class="program-description">{ &ctx.props().description }</div>
                    <div class="broadcast-schedule bg-red-100">
                        <p>{ format!("放送日: {}", ctx.props().broadcast_date) }</p>
                        <p>{ &ctx.props().episode }</p>
                    </div>
                    <div class="status">
                        <span class={classes!("status-label", status_class)}>{ status_text }</span>
                        <button>{ "更新" }</button>
                    </div>
                </div>
            </div>
        }
    }
}
