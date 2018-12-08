#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Index {
    id: i32,
    name: String
}
pub struct Chunk {
    id: i32,
    name: String
}

pub struct IndexesPage {
    indexes: Vec<Index>
}

pub struct ChunksPage {
    chunks: Vec<Chunk>
}

pub struct Model {
    indexesPage: IndexesPage,
    chunksPage: ChunksPage
}

pub enum Msg {
    Indexes,
    Chunks
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let index1 = Index {
            id: 1,
            name: "test1".to_string()
        };
        let index2 = Index {
            id: 2,
            name: "test2".to_string()
        };
        let indexesPage = IndexesPage {
            indexes: vec![index1, index2]
        };
        let chunksPage = ChunksPage {
            chunks: vec![]
        };
        Model {
            indexesPage: indexesPage,
            chunksPage: chunksPage
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Indexes => {
                true
            }
            Msg::Chunks => {
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <nav class="navbar navbar-inverse navbar-fixed-top",> {self.nav_sections()} </nav>

                 <div class="container-fluid",>
                    <div class="row",>
                       { self.view_side_bar() }
                       <div class="col-sm-9 col-sm-offset-3 col-md-10 col-md-offset-2 main",>
                       <h1 class="page-header",>{"Dashboard"}</h1>
                       {self.view_image_ribbon()}
                       </div>
                    </div>
                 </div>

                //<nav class="menu",>{ self.view_menu() }</nav>
                <table>
                  { self.view_rows() }
                </table>
                </>
        }
    }
}

impl Model {
    fn view_rows(&self) -> Html<Self> {
        let render = |idx| html! {
            <tr>{ idx }</tr>
                <tr><td>{"test"}</td><td>{"hello"}</td></tr> 
        };
        html! { // We use a fragment directly
                    <>
                        <tr>{"hi"}</tr>
                </>
            {for self.indexesPage.indexes.iter().map(|i| html! {
                <tr> <td> { i.id } </td> <td> { i.name.to_owned() } </td> <td><button onclick=|_| Msg::Indexes,>{ "IndexesPage" }</button></td></tr>
            })}
            { for (0..10).map(render) }
            { for (0..10).map(render) }
        }
    }
    fn view_menu(&self) -> Html<Self> {
        html! {
            <>
                <button onclick=|_| Msg::Indexes,>{ "IndexesPage" }</button>
                <button onclick=|_| Msg::Chunks,>{ "ChunksPage" }</button>
            </>
        }
    }
    fn nav_sections(&self) -> Html<Self> {
        html! {
      <div class="container-fluid",>
            { self.nav_section_1() }
      </div>
        }
    }

    fn nav_section_1(&self) -> Html<Self> {
        html! {
            <>
                <div class="navbar-header",>
                <button type="button", class="navbar-toggle collapsed", data-toggle="collapse", data-target="#navbar", aria-expanded="false", aria-controls="navbar",>
                <span class="sr-only",>{"Toggle navigation"}</span>
                <span class="icon-bar",></span>
                <span class="icon-bar",></span>
                <span class="icon-bar",></span>
                </button>
                <a class="navbar-brand",href="#",>{"Casync dashboard"}</a>
                </div>
            { self.nav_section_links() }
            </>
        }
    }

    fn nav_section_links(&self) -> Html<Self> {
        html! {
            <>
                <div id="navbar",class="navbar-collapse collapse",>
                <ul class="nav navbar-nav navbar-right",>
            { self.nav_section_links_ul_items() }
                </ul>
            <form class="navbar-form navbar-right",>
            <input type="text",class="form-control",placeholder="Search...",/>
            </form>
                </div>
            </>
        }
    }

    fn nav_section_links_ul_items(&self) -> Html<Self> {
        html! {
            <>
                <li><a href="#",>{"Dashboard"}</a></li>
                <li><a href="#",>{"Settings"}</a></li>
                <li><a href="#",>{"Profile"}</a></li>
                <li><a href="#",>{"Help"}</a></li>
            </>
        }
    }

    fn view_side_bar(&self) -> Html<Self> {
        html! {
            <div class="col-sm-3 col-md-2 sidebar",>
            { self.view_side_bar_sub_sections() }
            </div>
        }
    }

    fn view_side_bar_sub_sections(&self) -> Html<Self> {
        html! {
            <>
                <ul class="nav nav-sidebar",>
            { self.view_side_bar_sub_sections_1() }
                </ul>
                <ul class="nav nav-sidebar",>
            { self.view_side_bar_sub_sections_2() }
                </ul>
                 <ul class="nav nav-sidebar",>
            {self.view_side_bar_sub_sections_3() }
                </ul>
            </>
        }
    }

    fn view_side_bar_sub_sections_1(&self) -> Html<Self> {
        html! {
            <>
                <li class="active",> { self.view_side_bar_sub_sections_1_overview() } </li>
                <li><a href="#",>{"Reports"}</a></li>
                <li><a href="#",>{"Analytics"}</a></li>
                <li><a href="#",>{"Export"}</a></li>
            </>
        }
    }

    // Fix this
    fn view_side_bar_sub_sections_1_overview(&self) -> Html<Self> {
        html! {
            <>
                <a href="#",>{ self.view_side_bar_sub_sections_1_overview_span() }</a>
            </>
        }
    }

    fn view_side_bar_sub_sections_1_overview_span(&self) -> Html<Self> {
        html! {
            <>
                <span class="sr-only",>{"Overview"}</span>
            </>
        }
    }

    fn view_side_bar_sub_sections_2(&self) -> Html<Self> {
        html! {
            <>
                <li><a href="",>{"Nav item"}</a></li>
                <li><a href="",>{"Nav item again"}</a></li>
                <li><a href="",>{"One more nav"}</a></li>
                <li><a href="",>{"Another nav item"}</a></li>
                <li><a href="",>{"More navigation"}</a></li>
            </>
        }
    }

    fn view_side_bar_sub_sections_3(&self) -> Html<Self> {
        html! {
            <>
                <li><a href="",>{"Nav item again"}</a></li>
                <li><a href="",>{"One more nav"}</a></li>
                <li><a href="",>{"Another nav item"}</a></li>
            </>
        }
    }

    fn view_image_ribbon(&self) -> Html<Self> {
        html! {
            <div class="row placeholders",>
            { self.view_image_ribbon_1() }
            </div>
            <div class="col-xs-6 col-sm-3 placeholder",>
            { self.view_image_ribbon_2() }
            </div>
            <div class="col-xs-6 col-sm-3 placeholder",>
            { self.view_image_ribbon_3() }
            </div>
            <div class="col-xs-6 col-sm-3 placeholder",>
            { self.view_image_ribbon_4() }
            </div>
         </div>
        }
    }

    fn view_image_ribbon_1(&self) -> Html<Self> {
        html! {
            <>
                <div class="col-xs-6 col-sm-3 placeholder",>
                <img src="data:image/gif;base64,R0lGODlhAQABAIAAAHd3dwAAACH5BAAAAAAALAAAAAABAAEAAAICRAEAOw==",width="200",height="200",class="img-responsive",alt="Generic placeholder thumbnail",/>
                <h4>{"Label"}</h4>
                <span class="text-muted",>{"Something else"}</span>
            </>
        }
    }
    fn view_image_ribbon_2(&self) -> Html<Self> {
        html! {
            <>
                <img src="data:image/gif;base64,R0lGODlhAQABAIAAAHd3dwAAACH5BAAAAAAALAAAAAABAAEAAAICRAEAOw==",width="200",height="200",class="img-responsive",alt="Generic placeholder thumbnail",/>
                <h4>{"Label"}</h4>
                <span class="text-muted",>{"Something else"}</span>
            </>
        }
    }
    fn view_image_ribbon_3(&self) -> Html<Self> {
        html! {
            <>
                <img src="data:image/gif;base64,R0lGODlhAQABAIAAAHd3dwAAACH5BAAAAAAALAAAAAABAAEAAAICRAEAOw==",width="200",height="200",class="img-responsive",alt="Generic placeholder thumbnail",/>
                <h4>{"Label"}</h4>
                <span class="text-muted",>{"Something else"}</span>
            </>
        }
    }
    fn view_image_ribbon_4(&self) -> Html<Self> {
        html! {
            <>
                <img src="data:image/gif;base64,R0lGODlhAQABAIAAAHd3dwAAACH5BAAAAAAALAAAAAABAAEAAAICRAEAOw==",width="200",height="200",class="img-responsive",alt="Generic placeholder thumbnail",/>
                <h4>{"Label"}</h4>
                <span class="text-muted",>{"Something else"}</span>
            </>
        }
    }
}

