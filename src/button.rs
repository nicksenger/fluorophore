use std::cell::RefCell;
use std::rc::Rc;

use mox::mox;
use moxie_dom::{
    elements::{
        forms::Button,
        html::{button as btn, span},
    },
    prelude::*,
};

#[derive(Clone)]
pub enum ButtonType {
    CTA,
    Primary,
    Secondary,
    Warning,
    Action,
}

pub fn button<T: Fn(event::Click)>() -> ButtonBuilder<T> {
    ButtonBuilder::default()
}

pub struct ButtonBuilder<T> {
    text: Option<String>,
    button_type: Option<ButtonType>,
    onclick: Option<Rc<RefCell<T>>>,
    disabled: Option<bool>,
}

impl<T: Fn(event::Click)> Default for ButtonBuilder<T> {
    fn default() -> Self {
        Self {
            text: None,
            button_type: None,
            onclick: None,
            disabled: None,
        }
    }
}

impl<T: Fn(event::Click) + 'static> ButtonBuilder<T> {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = Some(button_type);
        self
    }

    pub fn on_click(mut self, handler: T) -> Self {
        self.onclick = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn build(self) -> Button {
        let button_type = self.button_type.as_ref().unwrap_or(&ButtonType::Primary);
        let btn_class = match button_type {
            ButtonType::CTA => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--cta",
            ButtonType::Primary => {
                "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--primary"
            }
            ButtonType::Secondary => {
                "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--secondary"
            }
            ButtonType::Warning => {
                "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--warning"
            }
            ButtonType::Action => {
                "spectrum-ActionButton spectrum-ActionButton--quiet spectrum-ActionGroup-item"
            }
        };
        let label_class = match button_type {
            ButtonType::Action => "spectrum-ActionButton-label",
            _ => "spectrum-Button-label",
        };
        if let Some(handler) = self.onclick.as_ref() {
            let h = handler.clone();
            return mox! {
              <btn class={btn_class} disabled={self.disabled.unwrap_or(false)} onclick={move |e| {
                    h.borrow()(e);
              }}>
                <span class={label_class}>
                  {% "{}", self.text.unwrap_or("".to_owned())}
                </span>
              </btn>
            };
        }
        mox! {
          <btn class={btn_class} disabled={self.disabled.unwrap_or(false)}>
            <span class={label_class}>
              {% "{}", self.text.unwrap_or("".to_owned())}
            </span>
          </btn>
        }
    }
}
