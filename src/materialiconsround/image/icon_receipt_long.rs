
pub struct IconReceiptLong {
  props: crate::Props,
}

impl yew::Component for IconReceiptLong {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><path d="M0,0h24v24H0V0z" fill="none"/><g><path d="M14,9h-4C9.45,9,9,8.55,9,8v0c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0C15,8.55,14.55,9,14,9z"/><path d="M14,12h-4c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0C15,11.55,14.55,12,14,12z"/><path d="M19.5,3.5L18,2l-1.5,1.5L15,2l-1.5,1.5L12,2l-1.5,1.5L9,2L7.5,3.5L6,2v14H4c-0.55,0-1,0.45-1,1v2c0,1.66,1.34,3,3,3h12 c1.66,0,3-1.34,3-3V2L19.5,3.5z M15,20H6c-0.55,0-1-0.45-1-1v-1h3h4h3V20z M19,19c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2 c0-0.55-0.45-1-1-1h-2h-2H8V5h11V19z"/><circle cx="17" cy="8" r="1"/><circle cx="17" cy="11" r="1"/></g></svg>
            </svg>
        }
    }
}


