
pub struct IconFlipCameraIos {
  props: crate::Props,
}

impl yew::Component for IconFlipCameraIos {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M20,5h-3.17L15,3H9L7.17,5H4C2.9,5,2,5.9,2,7v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V7C22,5.9,21.1,5,20,5z M20,19H4V7 h3.17h0.88l0.59-0.65L9.88,5h4.24l1.24,1.35L15.95,7h0.88H20V19z"/></g><g><path d="M12,17c-2.21,0-4-1.79-4-4h2l-2.5-2.5L5,13h2c0,2.76,2.24,5,5,5c0.86,0,1.65-0.24,2.36-0.62l-0.74-0.74 C13.13,16.87,12.58,17,12,17z"/></g><g><path d="M12,8c-0.86,0-1.65,0.24-2.36,0.62l0.74,0.73C10.87,9.13,11.42,9,12,9c2.21,0,4,1.79,4,4h-2l2.5,2.5L19,13h-2 C17,10.24,14.76,8,12,8z"/></g></g></g></svg>
            </svg>
        }
    }
}


