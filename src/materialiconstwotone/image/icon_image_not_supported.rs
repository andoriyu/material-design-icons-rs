
pub struct IconImageNotSupported {
  props: crate::Props,
}

impl yew::Component for IconImageNotSupported {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><g><path d="M7.83,5H19v11.17L7.83,5z M16.17,19l-2-2H6l3-4l2,2.72l0.84-1.05L5,7.83V19H16.17z" opacity=".3"/><path d="M5.83,3H19c1.1,0,2,0.9,2,2v13.17l-2-2V5H7.83L5.83,3z M20.49,23.31L18.17,21H5c-1.1,0-2-0.9-2-2V5.83L0.69,3.51L2.1,2.1 l1.49,1.49L5,5l8.11,8.11l2.69,2.69L19,19l1.41,1.41l1.49,1.49L20.49,23.31z M16.17,19l-2-2H6l3-4l2,2.72l0.84-1.05L5,7.83V19 H16.17z"/></g></g></svg>
            </svg>
        }
    }
}


