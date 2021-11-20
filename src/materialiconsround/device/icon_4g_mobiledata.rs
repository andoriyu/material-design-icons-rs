
pub struct Icon4gMobiledata {
  props: crate::Props,
}

impl yew::Component for Icon4gMobiledata {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M8,7L8,7C7.45,7,7,7.45,7,8v4H5V8c0-0.55-0.45-1-1-1h0C3.45,7,3,7.45,3,8v5c0,0.55,0.45,1,1,1h3v2c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-2h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H9V8C9,7.45,8.55,7,8,7z M17,12L17,12c0,0.55,0.45,1,1,1h1v2h-5V9 h6c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-6c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2v-3c0-0.55-0.45-1-1-1h-2 C17.45,11,17,11.45,17,12z"/></g></g></svg>
            </svg>
        }
    }
}


