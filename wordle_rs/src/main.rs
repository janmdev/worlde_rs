use std::ptr::null;
use std::str::FromStr;
use yew::prelude::*;
use gloo::events::EventListener;
use gloo::net::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use uuid::Uuid;

//use crate::error::Error;
//use crate::types::ErrorInfo;

enum Msg {
    Reset,
    Keydown(Event)
}


#[derive(Copy, Clone, PartialEq)]
struct LetterStr {
    value: char,
    letter_type: u8
}

impl LetterStr {
    fn create() -> LetterStr {
        LetterStr {
            value: ' ',
            letter_type: 0
        }
    }
}

struct Model {
    letters: [[LetterStr; 5]; 5],
    kbd_listener: Option<EventListener>,
    row: i32,
    column: i32,
    alhpabet: [char; 26],
    session: Uuid
}



impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            letters: [[LetterStr::create(); 5]; 5],
            kbd_listener: None,
            row: 0,
            column: 0,
            alhpabet: ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'],
            session: Uuid::default()
        }
    }
    
    

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {

        if !first_render {
            return;
        }

        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let session_key = local_storage.get_item("session").unwrap();
        gloo_console::log!(session_key.as_ref().unwrap().to_string().clone());
        if session_key == None
        {
            let new_key = Uuid::new_v4();
            self.session = new_key;
            let set_result = local_storage.set_item("session", &new_key.to_string().to_owned());
            match set_result{
                Ok(_) => {
                    let session_string = self.session.to_string().to_owned();
                    wasm_bindgen_futures::spawn_local(async move {
                        let session_string = session_string.clone();
                        let init_f = Request::put(format!("https://localhost:7257/words/sessions/{}", session_string).as_str())
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                        gloo_console::log!(init_f);
                    });
                }
                Err(err) => {
                    gloo_console::log!(err);
                }
            }


        } else {
            self.session = Uuid::from_str(&session_key.unwrap().to_owned()).unwrap().clone();
        }

        let document = gloo::utils::document();
        let onkey_pressed = _ctx.link().callback(|ev| Msg::Keydown(ev));
        let listener = EventListener::new(&document, "keydown", move |event| onkey_pressed.emit(event.clone()));
        self.kbd_listener.replace(listener);
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                let session_string = self.session.to_string().to_owned();
                let new_session = Uuid::new_v4();
                self.session = new_session;
                let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                local_storage.set_item("session", &new_session.to_string().to_owned());
                wasm_bindgen_futures::spawn_local(async move {
                    let session_string = session_string.clone();
                    let new_session = new_session.clone();
                    let init_f = Request::delete(&*format!("https://localhost:7257/words/sessions/{}", session_string).as_str())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    gloo_console::log!(init_f);

                    let init_f = Request::put(format!("https://localhost:7257/words/sessions/{}", new_session).as_str())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    gloo_console::log!(init_f);
                });

                true
            }
            Msg::Keydown(ev) => {
                //gloo_console::log!(ev);
                let event = ev.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                let key = event.key().chars().next().unwrap().to_uppercase().next().unwrap();
                if event.key() == "Enter"
                {
                    if self.column == 5
                    {
                        if self.row == 4
                    {
                        gloo_console::log!("end");
                    }
                    else
                    {
                        gloo_console::log!("check row");
                        self.column = 0;
                        self.row += 1;
                        self.letters[0][1].letter_type = 1;
                        self.letters[0][2].letter_type = 2;
                    }
                    }
                    
                }
                else if event.key() == "Backspace"
                {
                    if self.column > 0
                    {
                        self.letters[self.row as usize][(self.column - 1) as usize] = LetterStr::create();
                        self.column -= 1;
                    }
                } else if self.column <= 4{
                    if self.alhpabet.iter().any(|x| x == &key)
                    {
                        gloo_console::log!("{} : {} : {}", self.row, self.column, key.to_string());
                        self.letters[self.row as usize][self.column as usize].value = event.key().chars().next().unwrap().to_uppercase().next().unwrap();
                        self.column += 1;
                    }
                }
                
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        //let done = false;




        html! {
            <div class="main_box">
                <button onclick={link.callback(|_| Msg::Reset)}>{ "RESET" }</button>
                <p style="color: white;">{ self.session }</p>
                { 
                    for self.letters.iter()
                    .map(
                        |b| 
                            if self.letters.iter().position(|&x| x == *b).unwrap() as i32 == self.row
                            {
                                html! {<div class="outer_box" style="border: 1px solid white; border-radius: 3px;"> { 
                                    for b.iter()
                                    .map(
                                        |c| 
                                        match c.letter_type {
                                            1 => html! { <div class="inner_box" style="background-color: yellow;"> {c.value} </div> },
                                            2 => html! { <div class="inner_box" style="background-color: green;"> {c.value} </div> },
                                            _ => html! { <div class="inner_box"> {c.value} </div> }
                                        }
                                    )
                                 } </div> }
                            }
                            else
                            {
                                html! {<div class="outer_box"> {
                                    for b.iter()
                                .map(
                                    |c| 
                                    match c.letter_type {
                                        1 => html! { <div class="inner_box" style="background-color: yellow;"> {c.value} </div> },
                                        2 => html! { <div class="inner_box" style="background-color: green;"> {c.value} </div> },
                                        _ => html! { <div class="inner_box"> {c.value} </div> }
                                    }
                                )
                                } </div> }
                            }
                                
                        ) 
                    } 
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}