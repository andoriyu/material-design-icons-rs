
pub struct IconPlaylistRemove {
  props: crate::Props,
}

impl yew::Component for IconPlaylistRemove {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M13.71,21.3c0.39,0.39,1.02,0.39,1.41,0L17,19.41l1.89,1.89c0.39,0.39,1.02,0.39,1.41,0s0.39-1.02,0-1.41L18.41,18 l1.89-1.89c0.39-0.39,0.39-1.02,0-1.41s-1.02-0.39-1.41,0L17,16.59l-1.89-1.89c-0.39-0.39-1.02-0.39-1.41,0s-0.39,1.02,0,1.41 L15.59,18l-1.89,1.89C13.32,20.27,13.32,20.91,13.71,21.3z M14,11c0-0.55-0.45-1-1-1H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h9 C13.55,12,14,11.55,14,11z M14,7c0-0.55-0.45-1-1-1H4C3.45,6,3,6.45,3,7s0.45,1,1,1h9C13.55,8,14,7.55,14,7z M3,15 c0,0.55,0.45,1,1,1h5c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,14,3,14.45,3,15z"/></g></svg>
            </svg>
        }
    }
}


