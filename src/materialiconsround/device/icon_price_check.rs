
pub struct IconPriceCheck {
  props: crate::Props,
}

impl yew::Component for IconPriceCheck {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M11,13V9c0-0.55-0.45-1-1-1H6V6h4c0.55,0,1-0.45,1-1s-0.45-1-1-1H8.5c0-0.55-0.45-1-1-1s-1,0.45-1,1H5C4.45,4,4,4.45,4,5 v4c0,0.55,0.45,1,1,1h4v2H5c-0.55,0-1,0.45-1,1s0.45,1,1,1h1.5c0,0.55,0.45,1,1,1s1-0.45,1-1H10C10.55,14,11,13.55,11,13z"/><path d="M18.88,13.22l-4.95,4.95l-2.12-2.12c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l2.83,2.83 c0.39,0.39,1.02,0.39,1.41,0l5.66-5.66c0.39-0.39,0.39-1.02,0-1.41v0C19.9,12.83,19.27,12.83,18.88,13.22z"/></g></g></svg>
            </svg>
        }
    }
}


