
pub struct IconManageSearch {
  props: crate::Props,
}

impl yew::Component for IconManageSearch {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M6,9H3C2.45,9,2,8.55,2,8v0c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v0C7,8.55,6.55,9,6,9z M6,12H3c-0.55,0-1,0.45-1,1v0 c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v0C7,12.45,6.55,12,6,12z M19.88,18.29l-3.12-3.12c-0.86,0.56-1.89,0.88-3,0.82 c-2.37-0.11-4.4-1.96-4.72-4.31C8.6,8.33,11.49,5.5,14.87,6.07c1.95,0.33,3.57,1.85,4,3.78c0.33,1.46,0.01,2.82-0.7,3.9l3.13,3.13 c0.39,0.39,0.39,1.02,0,1.41l0,0C20.91,18.68,20.27,18.68,19.88,18.29z M17,11c0-1.65-1.35-3-3-3s-3,1.35-3,3s1.35,3,3,3 S17,12.65,17,11z M3,19h8c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H3c-0.55,0-1,0.45-1,1v0C2,18.55,2.45,19,3,19z"/></g></svg>
            </svg>
        }
    }
}


