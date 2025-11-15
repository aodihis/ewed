use yew::{html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct CarouselItemProps {
    pub children: Html,
}


pub struct CarouselItem;

impl Component for CarouselItem {
    type Message = ();
    type Properties = CarouselItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
       Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { ctx.props().children.clone() }
            </div>
        }
    }
}