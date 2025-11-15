use yew::{function_component, html, ChildrenWithProps, Html, Properties};
use crate::components::ui::carousel::CarouselItem;

#[derive(Properties, PartialEq)]
pub struct CarouselContentProps {
    pub current: usize,
    pub children: ChildrenWithProps<CarouselItem>,
}

#[function_component(CarouselContent)]
pub fn carousel_content(props: &CarouselContentProps) -> Html {
    html! {
        <div
            class="flex transition-transform duration-700 ease-in-out"
            style={format!("transform: translateX(-{}%);", props.current * 100)}
        >
            { for props.children.iter().enumerate().map(|(_, child)| html! {
                <div class="w-full flex-shrink-0">{ child }</div>
            })}
        </div>
    }
}