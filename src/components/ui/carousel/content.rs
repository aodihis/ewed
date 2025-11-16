use yew::{classes, function_component, html, ChildrenWithProps, Html, Properties};
use crate::components::ui::carousel::CarouselItem;

#[derive(Properties, PartialEq)]
pub struct CarouselContentProps {
    pub prev: usize,
    pub current: usize,
    pub children: ChildrenWithProps<CarouselItem>,
}

#[function_component(CarouselContent)]
pub fn carousel_content(props: &CarouselContentProps) -> Html {
    html! {
        <div
            class="carousel-content"
            style={format!("transform: translateX(-{}%);", 100)}
        >
            { for props.children.iter().enumerate().map(|(idx, child)| {
                let pos_class = if idx == props.current {
                    "active"
                } else if idx == props.prev {
                    "prev"
                } else {
                    ""
                };

                html! {
                    <div class={classes!("carousel-item", pos_class)}>{ child }</div>
                    // <div class="carousel-item">{ child }</div>
                }
            })}
        </div>
    }
}