use gloo::net::Error;
use gloo::net::http::Response;
use gloo_console::log;
use yew::prelude::*;
use gloo::events::EventListener;
use gloo::net::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use uuid::Uuid;
use serde_json::json;
use serde::{Deserialize, Serialize};


enum Msg {
    ResetReq,
    Keydown(Event),
    PutUuid(Uuid),
    SetUuid(Uuid),
    None(String),
    SendWord(Uuid,String),
    SetWord([u8;5]),
    Reset(Uuid)
}

enum GameState {
    Playing,
    Won,
    Lost
}


#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
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
    session: Uuid,
    state: GameState,
    message: String
}

async fn put_guid(guid: Uuid) -> Result<(), ()> {
    let session_string = guid.to_string().to_owned();
    let init_f = Request::put(format!("https://localhost:7257/words/sessions/{}", session_string).as_str())
        .send()
        .await;
    match init_f {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}


async fn post_word(guid_: Uuid, word: String) -> Result<[u8;5], String> {
    let guid_ = guid_.to_string().clone();
    let body = json!({
        "guid": guid_,
        "word": word
    });
    let response = Request::post("https://localhost:7257/words")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await;
        match(response) {
            Ok(result) => 
            {
                if result.ok() {
                    Ok(result.json().await.unwrap())
                } else {
                    Err(result.text().await.unwrap())
                }
            },
            Err(err_mess) => Err(err_mess.to_string())
            
        }
}

async fn reset_session(from: Uuid, to: Uuid) -> Result<Response, Error> {
    let from = from.to_string().clone();
    let to = to.to_string().clone();
    
    let body = json!({
        "src": from,
        "dst": to
    });
     Request::post("https://localhost:7257/words/sessions/reset")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
}

fn set_to_local_storage(letter_arr: [[LetterStr; 5]; 5], row: i32)
{
    let letter_json = json!(letter_arr).to_string();
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.set_item("letters", &letter_json.clone().to_owned());
    local_storage.set_item("row", &row.to_string().clone().to_owned());
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let session_key = local_storage.get_item("session").unwrap();
        let mut final_key = Uuid::default();
        if session_key != None
        {
            final_key = Uuid::parse_str(session_key.unwrap().as_str()).unwrap();
        }
        let mut letters_ls = [[LetterStr::create(); 5]; 5];
        let letters_json = local_storage.get_item("letters").unwrap();
        if letters_json != None
        {
            letters_ls = serde_json::from_str(letters_json.unwrap().as_str()).unwrap();
        }
        let mut row_bef: i32 = 0;
        let row_str = local_storage.get_item("row").unwrap();
        if row_str != None
        {
            row_bef = row_str.unwrap().parse::<i32>().unwrap();
        }

        Self {
            letters: letters_ls,
            kbd_listener: None,
            row: row_bef,
            column: 0,
            alhpabet: ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'],
            session: final_key,
            state: GameState::Playing,
            message: "".to_owned()
        }
        
    }
    


    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {

        if !first_render {
            return;
        }
        
         let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
         let session_key = local_storage.get_item("session").unwrap();
         
         if session_key == None
         {
             let new_key = Uuid::new_v4();
             self.update(_ctx, Msg::PutUuid(new_key));
         }else{
             self.update(_ctx, Msg::SetUuid(Uuid::parse_str(session_key.unwrap().as_str()).unwrap()));
         }

        let document = gloo::utils::document();
        let onkey_pressed = _ctx.link().callback(|ev| Msg::Keydown(ev));
        let listener = EventListener::new(&document, "keydown", move |event| onkey_pressed.emit(event.clone()));
        self.kbd_listener.replace(listener);
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ResetReq => {
                let new_session = Uuid::new_v4().clone();
                let curr_session = self.session.clone();
                _ctx.link().send_future(async move{
                    match reset_session(curr_session, new_session).await {
                        Ok(resp) => 
                        {
                            if resp.ok()
                            {
                                Msg::Reset(new_session)
                            }else {
                                Msg::None(resp.text().await.unwrap())
                            }
                            
                        }
                        Err(err_mess) => Msg::None(err_mess.to_string())
                    }
                });
                true
            }
            Msg::Reset(guid) => {
                self.letters = [[LetterStr::create();5];5];
                self.row = 0;
                self.column = 0;
                self.state = GameState::Playing;
                self.session = guid;
                self.message = "".to_string();
                set_to_local_storage(self.letters.clone(), self.row.clone());
                let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                local_storage.set_item("session", self.session.to_string().clone().as_str());
                true
            }
            Msg::Keydown(ev) => {
                let event = ev.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                let full_key = event.key();
                let key = event.key().chars().next().unwrap().to_uppercase().next().unwrap();
                if self.row == 5 && matches!(self.state,GameState::Playing) {
                    self.state = GameState::Lost;
                }
                if matches!(self.state, GameState::Won) || matches!(self.state, GameState::Lost)
                {
                    ()
                }
                else if event.key() == "Enter"
                {
                    if self.column == 5
                    {
                        gloo_console::log!("end");
                        log!("check row");
                        let mut words_char: [char;5] = [' ';5];
                        for i in 0..5 {
                            words_char[i] = self.letters[self.row as usize][i as usize].value.clone();
                        }
                        let word_to_send = words_char.iter().collect();
                        log!(format!("Sending {}", word_to_send));
                        self.update(_ctx, Msg::SendWord(self.session, word_to_send));
                        log!(format!("After send, row: {}", self.row));   

                        
                    }
                }
                else if event.key() == "Backspace"
                {
                    if self.column > 0
                    {
                        self.letters[self.row as usize][(self.column - 1) as usize] = LetterStr::create();
                        self.column -= 1;
                    }
                } else if full_key.len() == 1 && self.column <= 4{
                    if self.alhpabet.iter().any(|x| x == &key)
                    {
                        //gloo_console::log!("{} : {} : {}", self.row, self.column, key.to_string());
                        self.letters[self.row as usize][self.column as usize].value = event.key().chars().next().unwrap().to_uppercase().next().unwrap();
                        self.column += 1;
                    }
                }
                
                true
            }
            Msg::PutUuid(guid) => {
                log!("Putting");
                log!(guid.to_string().clone().as_str());
                let guid = guid.clone();
                _ctx.link().send_future(async move {
                    match put_guid(guid).await {
                        Ok(_) => Msg::SetUuid(guid),
                        Err(_) => Msg::None("".to_string())
                    }
                });
                true
            }
            Msg::SetUuid(guid) => {
                self.session = guid.clone();
                let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                local_storage.set_item("session", self.session.to_string().clone().as_str());
                true
            }
            Msg::SendWord(guid, word) => {
                log!("Send word");
                let guid = guid.clone();
                let word = word.clone();
                _ctx.link().send_future(async move {
                    match post_word(guid, word).await {
                        Ok(response) => {
                            Msg::SetWord(response)
                        }
                        Err(err_mess) => {
                            Msg::None(err_mess)
                        }
                    }
                });
                true
            }
            Msg::SetWord(response) => {
                log!("Set word");
                for i in 0..5 {
                    self.letters[self.row as usize][i as usize].letter_type = response[i];
                }
                let matches = self.letters[self.row as usize].iter().filter(|x| x.letter_type == 2).count();
                self.row += 1;
                self.column = 0;
                set_to_local_storage(self.letters.clone(), self.row.clone());
                log!(matches.to_string());
                if matches == 5{
                    self.state = GameState::Won;
                }
                true
            }
            Msg::None(mess) => {
                log!(mess.clone());
                self.message = mess.clone();
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
                <button onclick={link.callback(|_| Msg::ResetReq)}>{ "RESET" }</button>
                <p style="color:white;">{self.session.to_string()}</p>
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
                    <p style="color:white;">{&self.message}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}