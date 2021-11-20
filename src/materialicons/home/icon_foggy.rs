
pub struct IconFoggy {
  props: crate::Props,
}

impl yew::Component for IconFoggy {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M17.92,7.02C17.45,4.18,14.97,2,12,2C9.82,2,7.83,3.18,6.78,5.06C4.09,5.41,2,7.74,2,10.5C2,13.53,4.47,16,7.5,16h10 c2.48,0,4.5-2.02,4.5-4.5C22,9.16,20.21,7.23,17.92,7.02z M18,17.01c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1 S17.45,17.01,18,17.01z M7,20.01c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S6.45,20.01,7,20.01z M6,17.01h9c0.55,0,1,0.45,1,1l0,0 c0,0.55-0.45,1-1,1H6c-0.55,0-1-0.45-1-1l0,0C5,17.46,5.45,17.01,6,17.01z M10,20.01h7c0.55,0,1,0.45,1,1l0,0c0,0.55-0.45,1-1,1h-7 c-0.55,0-1-0.45-1-1l0,0C9,20.46,9.45,20.01,10,20.01z"/></svg>
            </svg>
        }
    }
}


