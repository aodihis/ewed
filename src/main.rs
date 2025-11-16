mod components;

use std::time::Duration;
use yew::prelude::*;
use crate::components::ui::button::Button;
use crate::components::ui::carousel::{Carousel, CarouselItem};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    // language=HTML
    html! {
        <div>
            <Button label="Counter" onclick={onclick} />
                <Carousel
                    auto_play={true}
                    auto_play_interval={Duration::from_secs(5)}
                    show_controls={true}
                    show_indicators={true}
                    class={Some("w-78 h-64")}
            >
                <CarouselItem>
                    <img class={Some("w-full h-64 object-cover")} src="https://flowbite.com/docs/images/carousel/carousel-1.svg" alt="Slide" />
                </CarouselItem>
                <CarouselItem>
                    <img class={Some("w-full h-64 object-cover")} src="https://flowbite.com/docs/images/carousel/carousel-2.svg" alt="Slide" />
                </CarouselItem>
                <CarouselItem>
                    <img class={Some("w-full h-64 object-cover")} src="https://flowbite.com/docs/images/carousel/carousel-3.svg" alt="Slide" />
                </CarouselItem>
                <CarouselItem>
                    <img class={Some("w-full h-64 object-cover")} src="https://flowbite.com/docs/images/carousel/carousel-4.svg" alt="Slide" />
                </CarouselItem>

            </Carousel>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}