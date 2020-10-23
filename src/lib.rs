use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use futures::{channel::mpsc, future::ready, stream::Stream, StreamExt};
use moxie::{load_once, once, state, Commit};
use mox::mox;
use moxie_dom::{
    elements::{
        forms::button,
        text_content::{div, Div},
    },
    prelude::*,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn begin() {
    moxie_dom::boot(document().body(), root);
}

pub fn mox_stream<State: 'static, Msg: 'static + Clone, OutStream>(
    initial_state: State,
    reducer: impl Fn(&State, Msg) -> State + 'static,
    operator: impl FnOnce(mpsc::UnboundedReceiver<Msg>) -> OutStream,
) -> (Commit<State>, impl Fn(Msg))
where
    OutStream: Stream<Item = Msg> + 'static,
{
    let (current_state, accessor) = state(|| initial_state);

    let dispatch = once(|| {
        let (action_producer, action_consumer): (
            mpsc::UnboundedSender<Msg>,
            mpsc::UnboundedReceiver<Msg>,
        ) = mpsc::unbounded();
        let p = Arc::new(Mutex::new(action_producer));
        let pc = p.clone();

        let (mut operated_action_producer, operated_action_consumer): (
            mpsc::UnboundedSender<Msg>,
            mpsc::UnboundedReceiver<Msg>,
        ) = mpsc::unbounded();

        let _ = load_once(move || {
            action_consumer.for_each(move |msg| {
                accessor.update(|cur| Some(reducer(cur, msg.clone())));
                let _ = operated_action_producer.start_send(msg);
                ready(())
            })
        });

        let _ = load_once(move || {
            operator(operated_action_consumer).for_each(move |msg| {
                let _ = pc.lock().unwrap().start_send(msg);
                ready(())
            })
        });

        move |msg| {
            let _ = p.lock().unwrap().start_send(msg);
        }
    });

    (current_state, dispatch)
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