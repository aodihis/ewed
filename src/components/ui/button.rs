#![allow(unused_variables)]
#![allow(dead_code)]
use yew::prelude::*;

// === ENUMS === //
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

// === PROPS === //
#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or(ButtonSize::Md)]
    pub size: ButtonSize,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>
}

// === COMPONENT === //
pub struct Button {
    props: ButtonProps,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let ButtonProps {
            label,
            onclick,
            variant,
            size,
            disabled,
            class
        } = &self.props;

        let base_classes = "";

        let variant_class = match variant {
            ButtonVariant::Primary => "btn btn-primary",
            ButtonVariant::Secondary => "btn btn-secondary",
            ButtonVariant::Danger => "btn btn-danger",
        };

        let size_class = match size {
            ButtonSize::Sm => "px-3 py-1 text-sm",
            ButtonSize::Md => "px-4 py-2 text-base",
            ButtonSize::Lg => "px-5 py-3 text-lg",
        };

        let disabled_class = if *disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            ""
        };

        let mut all_classes = format!("{} {} {} {}", base_classes, variant_class, size_class, disabled_class);
        if let Some(extra) = class {
            all_classes.push(' ');
            all_classes.push_str(extra);
        }

        html! {
            <button
                class={all_classes}
                {onclick}
                disabled={*disabled}
            >
                { if !label.is_empty() { html! { &**label } } else { html! {} } }
            </button>
        }
    }
}
