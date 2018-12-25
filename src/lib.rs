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
use yew::services::fetch::{FetchService, Request, Response, FetchTask};
use yew::services::ConsoleService;


pub struct IndexesView {
    indexes: Vec<ds::IndexItemRow>
}

pub struct ChunksView {
    chunks: Vec<ds::ChunkItemRow>
}

pub struct TagsView {
    tags: Vec<ds::TagItem>,
    newName: String,
}

pub struct DetailedChunksView {
    chunks: Vec<ds::ChunkItem>
}

pub struct Model {
    indexesView: IndexesView,
    chunksView: ChunksView,
    tagsView: TagsView,
    detailedChunksView: DetailedChunksView,
    activeView: ActiveView,
    link: ComponentLink<Model>,
    fetchService: FetchService,
    fetching: bool,
    consoleService: ConsoleService,
    fetchTask: Option<FetchTask>
}

pub enum ActiveView {
    Upload,
    Indexes,
    Chunks,
    Tags,
    DetailedChunks,
    Loading,
    TagNewInput
}

pub enum Msg {
    Upload,
    IndexesAll,
    ChunksAll,
    TagsAll,
    FetchedIndexes(Result<Vec<ds::IndexItemRow>, Error>),
    ChunksByIndexId(i32),
    FetchedChunkRows(Result<Vec<ds::ChunkItemRow>, Error>),
    FetchedDetailedChunks(Result<Vec<ds::ChunkItem>, Error>),
    FetchedTags(Result<Vec<ds::TagItem>, Error>),
    FetchErr,
    TagNewClick,
    TagNewInput(String),
    TagNewSubmit,
    TagNewSubmitCancel,
    TagNewCreated(Result<ds::TagItem, Error>)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let indexesView = IndexesView {
            indexes: vec![]
        };
        let chunksView = ChunksView {
            chunks: vec![]
        };
        let tagsView = TagsView {
            tags: vec![],
            newName: "".to_owned()
        };
        let detailedChunksView = DetailedChunksView {
            chunks: vec![]
        };
        Model {
            indexesView: indexesView,
            chunksView: chunksView,
            tagsView: tagsView,
            detailedChunksView: detailedChunksView,
            activeView: ActiveView::Upload,
            fetchService: FetchService::new(),
            fetching: false,
            link,
            consoleService: ConsoleService::new(),
            fetchTask: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Upload => {
                self.activeView = ActiveView::Upload;
                true
            },
            Msg::IndexesAll => {
                self.activeView = ActiveView::Indexes;
                self.fetching = true;
                let callback = self.link.send_back(move |response: Response<Json<Result<Vec<ds::IndexItemRow>, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchedIndexes(data)
                    } else {
                        Msg::FetchErr  // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::get("http://localhost:8001/indexes").body(Nothing).unwrap();
                let task = self.fetchService.fetch_binary(request, callback);
                self.fetchTask = Some(task);               
                true
            },
            Msg::ChunksAll => {
                self.activeView = ActiveView::Loading;
                self.fetching = true;
                let callback = self.link.send_back(move |response: Response<Json<Result<Vec<ds::ChunkItemRow>, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchedChunkRows(data)
                    } else {
                        Msg::FetchErr  // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::get("http://localhost:8001/chunks").body(Nothing).unwrap();
                let task = self.fetchService.fetch_binary(request, callback);
                self.fetchTask = Some(task);
                true
            },
            Msg::TagsAll => {
                self.activeView = ActiveView::Loading;
                self.fetching = true;
                let callback = self.link.send_back(move |response: Response<Json<Result<Vec<ds::TagItem>, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchedTags(data)
                    } else {
                        Msg::FetchErr  // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::get("http://localhost:8001/tags").body(Nothing).unwrap();
                let task = self.fetchService.fetch_binary(request, callback);
                self.fetchTask = Some(task);
                true
            },            
            Msg::ChunksByIndexId(index_id) => {
                self.activeView = ActiveView::Loading;
                self.fetching = true;
                let callback = self.link.send_back(move |response: Response<Json<Result<Vec<ds::ChunkItem>, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchedDetailedChunks(data)
                    } else {
                        Msg::FetchErr  // FIXME: Handle this error accordingly.
                    }
                });
                let mut url = "http://localhost:8001/chunks?index_id=".to_string();
                url.push_str(&format!("{}",index_id));
                let request = Request::get(&url).body(Nothing).unwrap();
                let task = self.fetchService.fetch_binary(request, callback);
                self.fetchTask = Some(task);
                true
            }, 
            Msg::FetchedIndexes(data) => {
                self.fetching = false;
                self.activeView = ActiveView::Indexes;
                self.indexesView = IndexesView {
                    indexes: data.unwrap()
                };
                true
            },           
            Msg::FetchedChunkRows(data) => {
                self.fetching = false;
                self.activeView = ActiveView::Chunks;
                self.chunksView = ChunksView {
                    chunks: data.unwrap()
                };
                //self.consoleService.log(&format!("Fetched chunks {}", data.unwrap().len()));
                true
            },
            Msg::FetchedDetailedChunks(data) => {
                self.fetching = false;
                self.activeView = ActiveView::DetailedChunks;
                self.detailedChunksView = DetailedChunksView {
                    chunks: data.unwrap()
                };
                //self.consoleService.log(&format!("Fetched chunks {}", data.unwrap().len()));
                true
            },
            Msg::FetchedTags(data) => {
                self.fetching = false;
                self.activeView = ActiveView::Tags;
                self.tagsView = TagsView {
                    tags: data.unwrap(),
                    newName: "".to_owned()
                };
                true
            },
            Msg::FetchErr => {
                self.fetching = false;
                println!("We got error while fetching data");
                self.consoleService.debug("Got err while fetching");
                true
            },
            Msg::TagNewClick => {
                self.activeView = ActiveView::TagNewInput;
                true
            },
            Msg::TagNewInput(value) => {
                self.activeView = ActiveView::TagNewInput;
                self.tagsView.newName = value;
                false
            },
            Msg::TagNewSubmit => {
                let tag_name = &self.tagsView.newName;
                self.consoleService.debug(&format!("Submitting new tag {}",tag_name));
                self.activeView = ActiveView::Loading;
                self.fetching = true;
                let callback = self.link.send_back(move |response: Response<Json<Result<ds::TagItem, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::TagNewCreated(data)
                    } else {
                        Msg::FetchErr  // FIXME: Handle this error accordingly.
                    }
                });
                let mut url = "http://localhost:8001/tags/new?name=".to_string();
                url.push_str(&format!("{}",tag_name));
                let request = Request::put(&url).body(Nothing).unwrap();
                let task = self.fetchService.fetch_binary(request, callback);
                self.fetchTask = Some(task);
                true
            },
            Msg::TagNewSubmitCancel => {
                self.tagsView.newName = "".to_string();
                self.activeView = ActiveView::Tags;
                true
            },
            Msg::TagNewCreated(data) => {
                self.tagsView.newName = "".to_string();
                self.activeView = ActiveView::Tags;
                self.tagsView.tags.push(data.unwrap());
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

            //{self.view_image_ribbon()}

            { self.main_view() }
            </div>
                </div>
                </div>
                </>
        }
    }
}

impl Model {
    fn main_view(&self) -> Html<Self> {
        match self.activeView {
            ActiveView::Upload => self.upload_view(),
            _ => self.view_table()
        }
    }
    fn upload_view(&self) -> Html<Self> {
        html! {
            <>
                <div class="row placeholders",>
                <table style="width:100%",>
                    <tr>
                        <td></td>
                        <td></td>
                        <td>{self.upload_index_view()}</td>
                    </tr>
                    <tr>
                        <td>{self.upload_blob_view()}</td>
                        <td>{self.upload_separator()}</td>
                        <td></td>
                    </tr>
                    <tr>
                        <td></td>
                        <td></td>
                        <td>{self.upload_chunks_view()}</td>
                    </tr>
                </table>
                </div>
            </>
        }
    }

    fn upload_blob_view(&self) -> Html<Self> {
        html!{
            <>
                    <input id="input-blob", type="file", name="blob", style="display:inline-block;",/>
                    <button id="btn-blob", value="Upload",> { "Upload blob" } </button><br/>
                    <progress id="progress-blob", style = "margin-left: -43%;margin-top: 3%; display:none",/>
            </>
        }
    }

    fn upload_index_view(&self) -> Html<Self> {
        html!{
            <>
                    <input id="input-index", type="file", name="index", style="display:inline-block;",/>
                    <button id="btn-index", value="Upload",> { "Upload index" } </button><br/>
                    <progress id="progress-index", style = "margin-left: -43%;margin-top: 3%; display:none",/>
            </>
        }
    }

    fn upload_chunks_view(&self) -> Html<Self> {
        html!{
            <>
                    <input id="input-chunks", type="file", name="chunks", style="display:inline-block;", multiple="multiple", disabled=true,/>
                    <button id="btn-chunks", value="Upload", disabled=true,> { "Upload chunks" } </button><br/>
                    <progress id="progress-chunks", style = "margin-left: -43%;margin-top: 3%; display:none",/>
            </>
        }
    }

    fn upload_separator(&self) -> Html<Self> {
        html! {
            <>
                <h1>{"|"}</h1>
            </>
        }
    }
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
            // This is not needed
            ActiveView::Upload => {
                html! {
                    <>
                        <h1> {" A upload chunks, indexes, blob view will be here "} </h1>
                    </>
                }
            },
            ActiveView::Indexes => {
                html! {
                    <>
                    {for self.indexesView.indexes.iter().map(|i| { 
                        let index_id = i.id;
                        html! {
                        <tr>
                            <td> { index_id } </td>
                            <td> { i.name.to_owned() } </td>
                            <td>{ i.creation_time.to_owned() }</td>
                            <td>{ i.accessed_time.to_owned() }</td>
                            <td>{ i.stats_confirmed_download_count }</td>
                            <td>{ i.stats_anonymous_download_count }</td>
                            <td><button onclick=|_| Msg::ChunksByIndexId(index_id),>{ "View Chunks" }</button></td>
                            </tr>
                    }})}
                    </>
                }
            },
            ActiveView::Chunks => {
                html! {
                    <>
                    {for self.chunksView.chunks.iter().map(|i| html! {
                        <tr>
                            <td> { i.id } </td>
                            <td> { i.index_id } </td>
                            <td> { i.name.to_owned() } </td>
                            <td>{ i.size }</td>
                            <td>{ i.creation_time.to_owned() }</td>
                            <td>{ i.accessed_time.to_owned() }</td>
                            <td>{ i.tags.len() }</td>
                            <td>{ i.stats_download_count }</td>
                            </tr>
                    })}
                    </>
                }
            },
            ActiveView::Tags | ActiveView::TagNewInput => {
                html! {
                    <>
                    {for self.tagsView.tags.iter().map(|i| html! {
                        <tr>
                            <td> { i.id } </td>
                            <td> { i.name.to_owned() } </td>
                            <td>{ i.creation_time.to_owned() }</td>
                            <td>{ i.accessed_time.to_owned() }</td>
                            </tr>
                    })}
                    </>
                }
            },
            ActiveView::DetailedChunks => {
                html! {
                    <>
                    {for self.detailedChunksView.chunks.iter().map(|i| html! {
                        <tr>
                            <td> { i.id } </td>
                            <td> { i.name.to_owned() } </td>
                            <td>{ i.size }</td>
                            <td>{ i.creation_time.to_owned() }</td>
                            <td>{ i.accessed_time.to_owned() }</td>
                            <td>{ i.tags.len() }</td>
                            <td>{ i.stats_download_count }</td>
                            </tr>
                    })}
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
            ActiveView::Upload => {
                html! {
                    <>
                        <h2 class="sub-header",>{"Upload"}</h2>
                    </>
                }
            },
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
            ActiveView::Tags => {
                html! {
                    <>
                        <h2 class="sub-header",>{"Tags"}</h2>
                        <button onclick=|_| Msg::TagNewClick,>{ "+ New Tag" }</button>
                    </>
                }
            },
            ActiveView::TagNewInput => {
                html! {
                    <>
                        <h2 class="sub-header",>{"Tags"}</h2>
                        <input type="text", name="tag_name", 
                        value=&(self.tagsView.newName.to_owned()),placeholder="New Tag name", 
                        oninput=|e| Msg::TagNewInput(e.value),/>
                        <button onclick=|_| Msg::TagNewSubmit,> {" Save "} </button>
                        <button onclick=|_| Msg::TagNewSubmitCancel,> {" Cancel "} </button>
                    </>
                }
            },
            ActiveView::DetailedChunks => {
                html! {
                    <>
                        <h2 class="sub-header",>{"Chunks in detail"}</h2>
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
        match self.activeView {
            ActiveView::Upload => {
                html!{
                    <>
                    </>
                }
            },
            ActiveView::Indexes => {
                html!{
                    <>
                        <tr>
                        <th>{"id"}</th>
                        <th>{"name"}</th>
                        <th>{"creation_time"}</th>
                        <th>{"accessed_time"}</th>
                        <th>{"# confirmed downloads"}</th>
                        <th>{"# anonymous downloads"}</th>
                        </tr>
                        </>
                }
            },
            ActiveView::Chunks  => {
                html! {
                    <>
                        <tr>
                        <th>{"id"}</th>
                        <th>{"index_id"}</th>
                        <th>{"name"}</th>
                        <th>{"size"}</th>
                        <th>{"creation_time"}</th>
                        <th>{"accessed_time"}</th>
                        <th>{"tags"}</th>
                        <th>{"# downloads"}</th>
                        </tr>
                    </>
                }
            },
            ActiveView::Tags | ActiveView::TagNewInput => {
                html! {
                    <>
                        <tr>
                        <th>{"id"}</th>
                        <th>{"name"}</th>
                        <th>{"creation_time"}</th>
                        <th>{"accessed_time"}</th>
                        </tr>
                    </>
                }
            },
            ActiveView::DetailedChunks => {
                html! {
                    <>
                        <tr>
                        <th>{"id"}</th>
                        <th>{"name"}</th>
                        <th>{"size"}</th>
                        <th>{"creation_time"}</th>
                        <th>{"accessed_time"}</th>
                        <th>{"tags"}</th>
                        <th>{"# downloads"}</th>
                        </tr>
                    </>
                }
            },
            ActiveView::Loading => {
                html! {
                    <>
                    </>
                }
            }
        }
    }
    // fn view_menu(&self) -> Html<Self> {
    //     html! {
    //         <>
    //             <button onclick=|_| Msg::IndexesAll,>{ "IndexesView" }</button>
    //             <button onclick=|_| Msg::ChunksAll,>{ "ChunksView" }</button>
    //             </>
    //     }
    // }

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
                <li class="active",><a href="#",>{"Overview"}</a></li>
                <li><a href="#", onclick=|_| Msg::Upload,>{"Upload"}</a></li>
                <li><a href="#", onclick=|_| Msg::IndexesAll,>{"Indexes"}</a></li>
                <li><a href="#", onclick=|_| Msg::ChunksAll,>{"Chunks"}</a></li>
                <li><a href="#", onclick=|_| Msg::TagsAll,>{"Tags"}</a></li>
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
            <div class="col-xs-6 col-sm-3 placeholder",>
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

