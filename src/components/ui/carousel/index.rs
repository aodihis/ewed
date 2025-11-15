use yew::prelude::*;
use std::time::Duration;
use gloo::timers::callback::Interval;
use crate::components::ui::carousel::content::CarouselContent;
use crate::components::ui::carousel::item::CarouselItem;
use crate::components::ui::icons::left_arrow::LeftArrow;
use crate::components::ui::icons::right_arrow::RightArrow;

#[derive(Properties, PartialEq, Clone)]
pub struct CarouselProps {
    pub children: ChildrenWithProps<CarouselItem>,
    #[prop_or(0)]
    pub start_index: usize,
    #[prop_or(true)]
    pub auto_play: bool,
    #[prop_or(Duration::from_secs(3))]
    pub auto_play_interval: Duration,
    #[prop_or(false)]
    pub show_indicators: bool,
    #[prop_or(false)]
    pub show_controls: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}


pub struct Carousel {
    current: usize,
    interval: Option<Interval>,
}

pub enum Msg {
    Next,
    Prev,
    SetIndex(usize),
}

impl Carousel {
    fn setup_timer(ctx: &Context<Self>) -> Interval {
        let link = ctx.link().clone();
        let interval = Interval::new(
            ctx.props().auto_play_interval.as_millis() as u32,
            move || link.send_message(Msg::Next),
        );
        interval
    }
}

impl Component for Carousel {
    type Message = Msg;
    type Properties = CarouselProps;

    fn create(ctx: &Context<Self>,) -> Self {
        let mut carousel = Self {
            current: ctx.props().start_index,
            interval: None,
        };

        if ctx.props().auto_play {
           carousel.interval = Some(Self::setup_timer(ctx));
        }

        carousel
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let len = ctx.props().children.len();

        // If autoplay is enabled, always reset the timer
        if ctx.props().auto_play {
            self.interval = Some(Self::setup_timer(ctx));
        }

        match msg {
            Msg::Next => {
                self.current = (self.current + 1) % len;
                true
            }
            Msg::Prev => {
                self.current = (self.current + len - 1) % len; // wrap backwards cleanly
                true
            }
            Msg::SetIndex(idx) => {
                if idx < len {
                    self.current = idx;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let len = ctx.props().children.len();
        let child = ctx.props().children.clone();
        html! {
            <div class={classes!(
                "relative",
                "overflow-hidden",
                ctx.props().class.clone().unwrap_or_default()
            )}>

                <CarouselContent current={self.current}>
                    {{child}}
                </CarouselContent>


                <CarouselControls
                    show={ctx.props().show_controls}
                    on_prev={ctx.link().callback(|_| Msg::Prev)}
                    on_next={ctx.link().callback(|_| Msg::Next)}
                />

                <CarouselIndicators
                    show={ctx.props().show_indicators}
                    len={len}
                    current={self.current}
                    on_change={ctx.link().callback(|idx| Msg::SetIndex(idx))}
                />

            </div>
        }

    }
}




#[derive(Properties, PartialEq)]
pub struct CarouselControlsProps {
    pub show: bool,
    pub on_prev: Callback<()>,
    pub on_next: Callback<()>,
}

#[function_component(CarouselControls)]
pub fn carousel_controls(props: &CarouselControlsProps) -> Html {
    if !props.show {
        return html! {};
    }

    html! {
        <>
            <button
                onclick={props.on_prev.reform(|_| ())}
                class="absolute left-2 top-1/2 -translate-y-1/2 bg-white/50 rounded-full p-2"
            >
                <LeftArrow />
            </button>

            <button
                onclick={props.on_next.reform(|_| ())}
                class="absolute right-2 top-1/2 -translate-y-1/2 bg-white/50 rounded-full p-2"
            >
                <RightArrow />
            </button>
        </>
    }
}


#[derive(Properties, PartialEq)]
pub struct CarouselIndicatorsProps {
    pub show: bool,
    pub len: usize,
    pub current: usize,
    pub on_change: Callback<usize>,
}

#[function_component(CarouselIndicators)]
pub fn carousel_indicators(props: &CarouselIndicatorsProps) -> Html {
    if !props.show {
        return html! {};
    }

    html! {
        <div class="absolute bottom-2 left-1/2 -translate-x-1/2 flex space-x-2">
            {
                for (0..props.len).map(|idx| {
                    let active = idx == props.current;

                    html! {
                        <button
                            key={idx}
                            onclick={props.on_change.reform(move |_| idx)}
                            class={classes!(
                                "w-3", "h-3", "rounded-full",
                                if active { "bg-white" } else { "bg-white/50" }
                            )}
                        />
                    }
                })
            }
        </div>
    }
}
