
pub struct Icon3gMobiledata {
  props: crate::Props,
}

impl yew::Component for Icon3gMobiledata {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M3,8L3,8c0,0.55,0.45,1,1,1h4v2H5c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h3v2H4c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1 h4c1.1,0,2-0.9,2-2v-1.5c0-0.83-0.67-1.5-1.5-1.5c0.83,0,1.5-0.67,1.5-1.5V9c0-1.1-0.9-2-2-2H4C3.45,7,3,7.45,3,8z M21,12v3 c0,1.1-0.9,2-2,2h-5c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h6c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h-6v6h5v-2h-1.5 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1H20C20.55,11,21,11.45,21,12z"/></g></g></svg>
            </svg>
        }
    }
}


