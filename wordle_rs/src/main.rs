use yew::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

enum Msg {
    AddOne,
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
    value: i64,
    letters: [[LetterStr; 5]; 5],
    kbd_listener: Option<EventListener>,
    row: i32,
    column: i32,
    alhpabet: [char; 26]
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            letters: [[LetterStr::create(); 5]; 5],
            kbd_listener: None,
            row: 0,
            column: 0,
            alhpabet: ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']
        }
    }
    

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        
        if !first_render {
            return;
        }
        let document = gloo::utils::document();
        let onkey_pressed = _ctx.link().callback(|ev| Msg::Keydown(ev));
        let listener = EventListener::new(&document, "keydown", move |event| onkey_pressed.emit(event.clone()));
        self.value += 1;
        self.kbd_listener.replace(listener);
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                gloo_console::log!("y");
                // the value has changed so we need to
                // re-render for it to appear on the page
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
        let mut done = false;

        html! {
            <div class="main_box">
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p style="color: white;">{ self.value }</p>
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