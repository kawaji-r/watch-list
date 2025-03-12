use yew::prelude::*;
use crate::components::header::Header;
use crate::components::search_bar::SearchBar;
use crate::components::program_list::ProgramList;
use crate::components::footer::Footer;

const GLOBAL_STYLES: &str = r#"
  /* グローバルスタイル */
  body {
      margin: 0;
      font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
      background-color: #f4f4f4;
      color: #333;
  }
  a {
      text-decoration: none;
      color: inherit;
  }
  header {
      background-color: #3f51b5;
      color: #fff;
      padding: 20px;
      text-align: center;
  }
  header .logo {
      font-size: 1.8em;
      font-weight: bold;
  }
  nav {
      margin-top: 10px;
  }
  nav a {
      margin: 0 15px;
      font-size: 1em;
      transition: color 0.3s ease;
  }
  nav a:hover {
      color: #c5cae9;
  }
  .container {
      max-width: 1200px;
      margin: 20px auto;
      padding: 0 20px;
  }
  .search-bar {
      text-align: center;
      margin-bottom: 30px;
  }
  .search-bar input[type="text"] {
      width: 300px;
      padding: 10px;
      font-size: 16px;
      border: 1px solid #ccc;
      border-radius: 25px;
      transition: box-shadow 0.3s ease;
  }
  .search-bar input[type="text"]:focus {
      outline: none;
      box-shadow: 0 0 5px rgba(63, 81, 181, 0.5);
  }
  .program-list {
      display: flex;
      flex-wrap: wrap;
      gap: 20px;
      justify-content: center;
  }
  .program-card {
      background-color: #fff;
      border-radius: 8px;
      overflow: hidden;
      box-shadow: 0 2px 8px rgba(0,0,0,0.1);
      width: 300px;
      transition: transform 0.3s ease;
  }
  .program-card:hover {
      transform: translateY(-5px);
  }
  .program-thumbnail {
      width: 100%;
      height: 180px;
      object-fit: cover;
  }
  .program-content {
      padding: 15px;
  }
  .program-title {
      font-size: 1.2em;
      font-weight: bold;
      margin-bottom: 8px;
  }
  .program-description {
      font-size: 0.9em;
      color: #666;
      margin-bottom: 15px;
  }
  .broadcast-schedule p {
      margin: 5px 0;
      font-size: 0.85em;
  }
  .status {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-top: 10px;
  }
  .status .status-label {
      font-size: 0.9em;
      padding: 5px 10px;
      border-radius: 15px;
      color: #fff;
  }
  .status .not-watched {
      background-color: #e74c3c;
  }
  .status .watched {
      background-color: #2ecc71;
  }
  .status button {
      padding: 5px 10px;
      font-size: 0.9em;
      border: none;
      border-radius: 5px;
      background-color: #3f51b5;
      color: #fff;
      cursor: pointer;
      transition: background-color 0.3s ease;
  }
  .status button:hover {
      background-color: #303f9f;
  }
  footer {
      background-color: #333;
      color: #fff;
      text-align: center;
      padding: 15px;
      margin-top: 40px;
  }
  @media (max-width: 768px) {
      .program-list {
          flex-direction: column;
          align-items: center;
      }
      .program-card {
          width: 90%;
      }
  }
"#;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
              <style>{ GLOBAL_STYLES }</style>
              <Header />
              <div class="container">
                  <SearchBar />
                  <ProgramList />
              </div>
              <Footer />
            </>
        }
    }
}
