use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct RightArrowProps {
    #[prop_or(24)]
    pub size: u32,

    #[prop_or_default]
    pub class: Option<String>,

    #[prop_or_else(|| "currentColor".into())]
    pub color: String,
}

#[function_component]
pub fn RightArrow(props: &RightArrowProps) -> Html {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width={props.size.to_string()}
            height={props.size.to_string()}
            class={props.class.clone().unwrap_or_default()}
            viewBox="0 0 24 24"
        >
            <path
                fill="none"
                stroke={props.color.clone()}
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 12h14m-7-7l7 7l-7 7"/>
        </svg>
    }
}
