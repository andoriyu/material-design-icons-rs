
pub struct IconNearbyOff {
  props: crate::Props,
}

impl yew::Component for IconNearbyOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M21.41,13.42L18.83,16l-1.81-1.81L19.2,12L12,4.8L9.81,6.99L8,5.17l2.58-2.58c0.78-0.78,2.05-0.78,2.83,0l8,8 C22.2,11.37,22.2,12.63,21.41,13.42z M20.48,21.9L20.48,21.9c-0.39,0.39-1.02,0.39-1.41,0L16,18.83l-2.58,2.58 c-0.78,0.78-2.05,0.78-2.83,0l-8-8c-0.78-0.78-0.78-2.05,0-2.83L5.17,8L2.1,4.93c-0.39-0.39-0.39-1.02,0-1.41l0,0 c0.39-0.39,1.02-0.39,1.41,0l16.98,16.97C20.87,20.87,20.87,21.51,20.48,21.9z M14.19,17.02l-1.39-1.39l-0.09,0.09 c-0.39,0.39-1.02,0.39-1.42,0l-3.01-3.01c-0.39-0.39-0.39-1.02,0-1.41l0.09-0.09l-1.4-1.39L4.8,12l7.2,7.2L14.19,17.02z M15.71,11.29l-3.01-3.01c-0.39-0.39-1.02-0.39-1.41,0L11.2,8.38l4.42,4.42l0.09-0.09C16.1,12.32,16.1,11.68,15.71,11.29z"/></g></g></svg>
            </svg>
        }
    }
}


