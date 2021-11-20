
pub struct IconRowing {
  props: crate::Props,
}

impl yew::Component for IconRowing {
 type Properties = crate::Props;
    type Message = ();

    fn create(props: Self::Properties, _: yew::prelude::ComponentLink<Self>) -> Self
    {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> yew::prelude::ShouldRender
    {
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::prelude::ShouldRender
    {
        false
    }

    fn view(&self) -> yew::prelude::Html
    {
        yew::prelude::html! {
            <svg
                class=self.props.class.unwrap_or("")
                width=self.props.size.unwrap_or(24).to_string()
                height=self.props.size.unwrap_or(24).to_string()
                viewBox="0 0 24 24"
                fill=self.props.fill.unwrap_or("none")
                stroke=self.props.color.unwrap_or("currentColor")
                stroke-width=self.props.stroke_width.unwrap_or(2).to_string()
                stroke-linecap=self.props.stroke_linecap.unwrap_or("round")
                stroke-linejoin=self.props.stroke_linejoin.unwrap_or("round")
            >
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M8.5,14.5L4,19l1.5,1.5L9,17h2L8.5,14.5z M15,1c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S16.1,1,15,1z M21,21.01L18,24 l-2.99-3.01V19.5l-7.1-7.09C7.6,12.46,7.3,12.48,7,12.48v-2.16c1.66,0.03,3.61-0.87,4.67-2.04l1.4-1.55 C13.42,6.34,14.06,6,14.72,6h0.03C15.99,6.01,17,7.02,17,8.26v5.75c0,0.84-0.35,1.61-0.92,2.16l-3.58-3.58v-2.27 c-0.63,0.52-1.43,1.02-2.29,1.39L16.5,18H18L21,21.01z"/></g></g></g></svg>
            </svg>
        }
    }
}


