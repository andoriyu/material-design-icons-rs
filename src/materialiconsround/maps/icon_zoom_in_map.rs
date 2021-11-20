
pub struct IconZoomInMap {
  props: crate::Props,
}

impl yew::Component for IconZoomInMap {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M3,8c0,0.55,0.45,1,1,1l4,0c0.55,0,1-0.45,1-1l0-4c0-0.55-0.45-1-1-1S7,3.45,7,4l0,1.59L4.62,3.21 c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L5.59,7L4,7C3.45,7,3,7.45,3,8z M20,7h-1.59l2.38-2.38 c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0L17,5.59V4c0-0.55-0.45-1-1-1c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h4 c0.55,0,1-0.45,1-1S20.55,7,20,7z M4,17h1.59l-2.38,2.38c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0L7,18.41L7,20 c0,0.55,0.45,1,1,1s1-0.45,1-1l0-4c0-0.55-0.45-1-1-1l-4,0c-0.55,0-1,0.45-1,1C3,16.55,3.45,17,4,17z M21,16c0-0.55-0.45-1-1-1h-4 c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-1.59l2.38,2.38c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L18.41,17H20C20.55,17,21,16.55,21,16z"/></g></svg>
            </svg>
        }
    }
}


