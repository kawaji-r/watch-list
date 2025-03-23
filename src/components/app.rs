use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::program_list::ProgramList;
use yew::prelude::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
          <Header />
          <div class="container mx-auto p-6">
              <ProgramList />
          </div>
          <Footer />
        </>
    }
}
