use std::{cell::RefCell, rc::Rc};

use futures::{future::ready, StreamExt};
use mox::mox;
use moxie_dom::{
    elements::{
        forms::button,
        text_content::{div, Div},
    },
    prelude::*,
};
use moxie_streams::mox_stream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn begin() {
    moxie_dom::boot(document().body(), root);
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    Double,
}

#[topo::nested]
fn root() -> Div {
    let (ct, dispatch) = mox_stream(
        Rc::new(RefCell::new(0)),
        |state, msg| match msg {
            Msg::Increment => {
                *(state.borrow_mut()) += 1;
                state.clone()
            },
            Msg::Decrement => {
                *(state.borrow_mut()) -= 1;
                state.clone()
            },
            Msg::Double => {
                *(state.borrow_mut()) *= 2;
                state.clone()
            }
        },
        |stream| stream.filter(|msg| match msg {
            Msg::Increment => ready(true),
            _ => ready(false)
        }).map(|_| Msg::Double),
    );

    let mut root = div();

    root = root.child(mox! { <div>{% "hello world from moxie! ({})", ct.borrow() }</div> });
    root = root.child(mox! {
        <button type="button" onclick={move |_| {dispatch(Msg::Increment);}}>
            "increment"
        </button>
    });

    for t in &["first", "second", "third"] {
        root = root.child(mox! { <div>{% "{}", t }</div> });
    }

    root.build()
}