
pub struct IconSensorDoor {
  props: crate::Props,
}

impl yew::Component for IconSensorDoor {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M18,4v16H6V4H18 M15.5,10.5c-0.83,0-1.5,0.67-1.5,1.5s0.67,1.5,1.5,1.5c0.83,0,1.5-0.67,1.5-1.5 S16.33,10.5,15.5,10.5z" opacity=".3"/><path d="M18,4v16H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v18h16V4C20,2.9,19.1,2,18,2L18,2z M15.5,10.5c-0.83,0-1.5,0.67-1.5,1.5 s0.67,1.5,1.5,1.5c0.83,0,1.5-0.67,1.5-1.5S16.33,10.5,15.5,10.5z"/></g></svg>
            </svg>
        }
    }
}


