
pub struct Icon30fpsSelect {
  props: crate::Props,
}

impl yew::Component for Icon30fpsSelect {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M4,5L4,5c0,0.55,0.45,1,1,1h4v2H6C5.45,8,5,8.45,5,9v0c0,0.55,0.45,1,1,1h3v2H5c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h4 c1.1,0,2-0.9,2-2v-1.5C11,9.67,10.83,9,10,9c0.83,0,1-0.67,1-1.5V6c0-1.1-0.9-2-2-2H5C4.45,4,4,4.45,4,5z M18,4c1.1,0,2,0.9,2,2v6 c0,1.1-0.9,2-2,2h-3c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2H18z M18,6h-3v6h3V6z M4,22L4,22c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1 h0c0.55,0,1,0.45,1,1v3C5,21.55,4.55,22,4,22z M8,22L8,22c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v3 C9,21.55,8.55,22,8,22z M12,22L12,22c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v3C13,21.55,12.55,22,12,22z M20,22h-4c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v3C21,21.55,20.55,22,20,22z"/></g></g></svg>
            </svg>
        }
    }
}


