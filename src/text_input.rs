use std::cell::RefCell;
use std::rc::Rc;

use mox::mox;
use moxie_dom::{
    elements::{
        html::{input, label},
        text_content::{div, Div},
    },
    prelude::*,
};
use wasm_bindgen::JsCast;

pub fn text_input<T: Fn(event::Input)>() -> TextInputBuilder<T> {
    TextInputBuilder::default()
}

pub struct TextInputBuilder<T> {
    input_type: Option<String>,
    label: Option<String>,
    placeholder: Option<String>,
    value: String,
    onchange: Option<Rc<RefCell<T>>>,
}

impl<T: Fn(event::Input)> Default for TextInputBuilder<T> {
    fn default() -> Self {
        Self {
            input_type: None,
            label: None,
            placeholder: None,
            value: "".to_owned(),
            onchange: None,
        }
    }
}

impl<T: Fn(event::Input) + 'static> TextInputBuilder<T> {
    pub fn input_type(mut self, x: impl Into<String>) -> Self {
        self.input_type = Some(x.into());
        self
    }

    pub fn label(mut self, x: impl Into<String>) -> Self {
        self.label = Some(x.into());
        self
    }

    pub fn placeholder(mut self, x: impl Into<String>) -> Self {
        self.placeholder = Some(x.into());
        self
    }

    pub fn value(mut self, x: impl Into<String>) -> Self {
        self.value = x.into();
        self
    }

    pub fn oninput(mut self, handler: T) -> Self {
        self.onchange = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn build(self) -> Div {
        print!("built!");
        let l = self.label.as_ref().map(|s| s.as_str()).unwrap_or("");
        let input_type = self
            .input_type
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("text");
        let placeholder = self.placeholder.as_ref().map(|s| s.as_str()).unwrap_or("");
        if let Some(handler) = self.onchange.as_ref() {
            let h = handler.clone();
            return mox! {
              <div class="spectrum-Form-item">
                <label class="spectrum-Form-itemLabel spectrum-FieldLabel--left" for={format!("{}{}-input", l, placeholder)}>
                  {% "{}", l}
                </label>
                <div class="spectrum-Form-itemField">
                  <div class="spectrum-TextField">
                    <input
                      class="spectrum-Textfield-input"
                      type={input_type}
                      placeholder={placeholder}
                      id={format!("{}{}-input", l, placeholder)}
                      value={&self.value}
                      oninput={move |ev| {
                        h.borrow()(ev);
                      }}
                    />
                  </div>
                </div>
              </div>
            };
        }
        mox! {
          <div class="spectrum-Form-item">
            <label class="spectrum-Form-itemLabel spectrum-FieldLabel--left" for={format!("{}{}-input", l, placeholder)}>
              {% "{}", l}
            </label>
            <div class="spectrum-Form-itemField">
              <div class="spectrum-Textfield">
                <input
                  class="spectrum-Textfield-input"
                  type={input_type}
                  placeholder={placeholder}
                  id={format!("{}{}-input", l, placeholder)}
                />
              </div>
            </div>
          </div>
        }
    }
}
