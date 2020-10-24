use mox::mox;
use moxie_dom::{
    elements::{
        forms::Button as Btn,
        html::{button as btn, span},
    },
    prelude::*,
};

pub enum ButtonType {
    CTA,
    Primary,
    Secondary,
    Warning,
    Action,
}

struct Button {
    text: String,
    button_type: ButtonType,
    on_click: fn(event::Click),
    disabled: bool,
}

fn button() -> ButtonBuilder {
    ButtonBuilder::default()
}

struct ButtonBuilder {
    text: Option<String>,
    button_type: Option<ButtonType>,
    on_click: Option<fn(event::Click)>,
    disabled: Option<bool>,
}

fn no_op<T>(_: T) {}

impl Default for ButtonBuilder {
  fn default() -> Self {
    Self {
      text: None,
      button_type: None,
      on_click: None,
      disabled: None,
    }
  }
}

impl ButtonBuilder {
  fn build(self) -> Button {
    Button {
      text: self.text.unwrap_or("".to_owned()),
      button_type: self.button_type.unwrap_or(ButtonType::CTA),
      on_click: self.on_click.unwrap_or(|_| {}),
      disabled: self.disabled.unwrap_or(false)
    }
  }
}

// #[topo::nested]
// pub fn button<T>(text: &str, button_type: ButtonType, on_click: T, disabled: bool) -> Button
// where
//     T: Fn(event::Click) + 'static,
// {
//     let btn_class = match button_type {
//         ButtonType::CTA => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--cta",
//         ButtonType::Primary => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--primary",
//         ButtonType::Secondary => {
//             "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--secondary"
//         }
//         ButtonType::Warning => "spectrum-Button spectrum-ButtonGroup-item spectrum-Button--warning",
//         ButtonType::Action => {
//             "spectrum-ActionButton spectrum-ActionButton--quiet spectrum-ActionGroup-item"
//         }
//     };
//     let label_class = match button_type {
//         ButtonType::Action => "spectrum-ActionButton-label",
//         _ => "spectrum-Button-label",
//     };
//     mox! {
//       <btn class={btn_class} onclick={on_click} disabled={disabled}>
//         <span class={label_class}>
//           {% "{}", text}
//         </span>
//       </btn>
//     }
// }
