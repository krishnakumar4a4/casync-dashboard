#[macro_use]
extern crate yew;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate failure;

use yew::prelude::*;
mod db;
mod ds;
use failure::Error;
use chrono::{DateTime, Utc};
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, Request, Response};
use yew::services::ConsoleService;


pub struct IndexesView {
    indexes: Vec<ds::IndexItem>
}

pub struct ChunksView {
    chunks: Vec<ds::ChunkItem>
}

pub struct Model {
    indexesView: IndexesView,
    chunksView: ChunksView,
    activeView: ActiveView,
    link: ComponentLink<Model>,
    fetchService: FetchService,
    fetching: bool,
    consoleService: ConsoleService
}

pub enum ActiveView {
    Indexes,
    Chunks,
    Loading
}

pub enum Msg {
    Indexes,
    Chunks,
    FetchedIndexes(Result<String, Error>),
    FetchedChunks,
//    FetchedChunks(Result<String, Error>),
    FetchErr
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let index1 = ds::IndexItem {
            id: 1,
            name: "test1".to_string(),
            path: "test1".to_string(),
            chunks: vec![],
            creation_time: "time".to_string(),
            accessed_time: "time".to_string(),
            stats_confirmed_download_count: 10,
            stats_anonymous_download_count: 11
        };
        let index2 = ds::IndexItem {
            id: 2,
            name: "test2".to_string(),
            path: "test2".to_string(),
            chunks: vec![],
            creation_time: "time".to_string(),
            accessed_time: "time".to_string(),
            stats_confirmed_download_count: 0,
            stats_anonymous_download_count: 1
        };
        let indexesView = IndexesView {
            indexes: vec![index1, index2]
        };
        let chunksView = ChunksView {
            chunks: vec![]
        };
        Model {
            indexesView: indexesView,
            chunksView: chunksView,
            activeView: ActiveView::Indexes,
            fetchService: FetchService::new(),
            fetching: false,
            link,
            consoleService: ConsoleService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Indexes => {
                self.activeView = ActiveView::Indexes;
                true
            },
            Msg::Chunks => {
                self.activeView = ActiveView::Loading;
                self.fetching = true;
                true
            },
            Msg::FetchedChunks => {
                self.fetching = false;
                self.activeView = ActiveView::Chunks;
//                let chunks: Vec<ds::ChunkItem> = serde_json::from_str(&data.unwrap()).unwrap();
                self.consoleService.log("Fetched chunks");
                true
            },
            Msg::FetchedIndexes(data) => {
                self.fetching = false;
     //           println!("We got some data for indexes of len {}", Json(data).unwrap().len());
                true
            },
            Msg::FetchErr => {
                self.fetching = false;
                println!("We got error while fetching data");
                self.consoleService.debug("Got err while fetching");
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                // Top Navigation bar
                <nav class="navbar navbar-inverse navbar-fixed-top",>
            {self.nav_sections()}
            </nav>

                // Side nav bar
                <div class="container-fluid",>
                <div class="row",>
            { self.view_side_bar() }

            // Main view with table
            <div class="col-sm-9 col-sm-offset-3 col-md-10 col-md-offset-2 main",>
                <h1 class="page-header",>{"Dashboard"}</h1>

            {self.view_image_ribbon()}

            { self.view_table() }

            </div>
                </div>
                </div>
                </>
        }
    }
}

impl Model {
    fn view_table(&self) -> Html<Self> {
        html!{
            <>
                { self.get_table_title() }
                <div class="table-responsive",>
                <table class="table table-striped",>
                <thead>
            { self.view_table_header() }
            </thead>
                <tbody>
            { self.get_table_view() }
                </tbody>
                </table>
                </div>
                </>
        }
    }

    fn get_table_view(&self) -> Html<Self> {
        match self.activeView {
            ActiveView::Indexes => {
                html! {
                    <>
                    {for self.indexesView.indexes.iter().map(|i| html! {
                        <tr>
                            <td> { i.id } </td>
                            <td> { i.name.to_owned() } </td>
                            <td>{ i.path.to_owned() }</td>
                            <td>{ i.creation_time.to_owned() }</td>
                            <td>{ i.accessed_time.to_owned() }</td>
                            <td>{ i.stats_confirmed_download_count }</td>
                            <td>{ i.stats_anonymous_download_count }</td>
                            <td><button onclick=|_| Msg::Chunks,>{ "ChunksView" }</button></td>
                            </tr>
                    })}
                    </>
                }
            },
            ActiveView::Chunks => {
                html! {
                    <>
                        </>
                }
            },
            ActiveView::Loading => {
                html! {
                    <>
                        <tr> { "Loading .." } </tr>
                        </>
                }
            }
        }
    }

    fn get_table_title(&self) -> Html<Self> {
        match self.activeView {
            ActiveView::Indexes => {
                html! {
                    <>
                        <h2 class="sub-header",>{"Indexes"}</h2>
                    </>
                }
            },
            ActiveView::Chunks => {
                html! {
                    <>
                        <h2 class="sub-header",>{"Chunks"}</h2>
                    </>
                }
            },
            ActiveView::Loading => {
                html! {
                    <>
                        <h2 class="sub-header",>{"Loading"}</h2>
                        </>
                }
            }
        }
    }
   fn view_table_header(&self) -> Html<Self> {
        html!{
            <>
                <tr>
                <th>{"id"}</th>
                <th>{"name"}</th>
                <th>{"path"}</th>
                <th>{"creation_time"}</th>
                <th>{"accessed_time"}</th>
                <th>{"# confirmed downloads"}</th>
                <th>{"# anonymous downloads"}</th>
                </tr>
                </>
        }
    }
    fn view_menu(&self) -> Html<Self> {
        html! {
            <>
                <button onclick=|_| Msg::Indexes,>{ "IndexesView" }</button>
                <button onclick=|_| Msg::Chunks,>{ "ChunksView" }</button>
                </>
        }
    }

    // Top navigation bar related views
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


    // Side bar related views
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
                <li><a href="#", onclick=|_| Msg::Indexes,>{"Indexes"}</a></li>
                <li><a href="#", onclick=|_| Msg::Chunks,>{"Chunks"}</a></li>
                <li><a href="#",>{"Tags"}</a></li>
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

    // Image ribbon related views
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

