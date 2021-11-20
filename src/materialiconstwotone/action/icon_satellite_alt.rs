
pub struct IconSatelliteAlt {
  props: crate::Props,
}

impl yew::Component for IconSatelliteAlt {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g opacity=".3"><path d="M6.6,19.32l-1.06,1.06L2,16.85l1.06-1.06L6.6,19.32z M8.72,17.2l-1.06,1.06l-3.54-3.54l1.06-1.06L8.72,17.2z M13.67,12.25 l-1.41,1.41l-3.54-3.54l1.41-1.41L13.67,12.25z M18.26,7.66L17.2,8.72l-3.54-3.54l1.06-1.06L18.26,7.66z M20.38,5.54L19.32,6.6 l-3.54-3.54L16.85,2L20.38,5.54z"/></g><g><path d="M15.44,0.59l-3.18,3.18c-0.78,0.78-0.78,2.05,0,2.83l1.24,1.24l-0.71,0.71L11.55,7.3c-0.78-0.78-2.05-0.78-2.83,0 L7.3,8.72c-0.78,0.78-0.78,2.05,0,2.83l1.24,1.24l-0.71,0.71L6.6,12.25c-0.78-0.78-2.05-0.78-2.83,0l-3.18,3.18 c-0.78,0.78-0.78,2.05,0,2.83l3.54,3.54c0.78,0.78,2.05,0.78,2.83,0l3.18-3.18c0.78-0.78,0.78-2.05,0-2.83l-1.24-1.24l0.71-0.71 l1.24,1.24c0.78,0.78,2.05,0.78,2.83,0l1.41-1.41c0.78-0.78,0.78-2.05,0-2.83L13.84,9.6l0.71-0.71l1.24,1.24 c0.78,0.78,2.05,0.78,2.83,0l3.18-3.18c0.78-0.78,0.78-2.05,0-2.83l-3.54-3.54C17.48-0.2,16.22-0.2,15.44,0.59z M6.6,19.32 l-1.06,1.06L2,16.85l1.06-1.06L6.6,19.32z M8.72,17.2l-1.06,1.06l-3.54-3.54l1.06-1.06L8.72,17.2z M13.67,12.25l-1.41,1.41 l-3.54-3.54l1.41-1.41L13.67,12.25z M18.26,7.66L17.2,8.72l-3.54-3.54l1.06-1.06L18.26,7.66z M20.38,5.54L19.32,6.6l-3.54-3.54 L16.85,2L20.38,5.54z M21,14l2,0c0,4.97-4.03,9-9,9l0-2C17.87,21,21,17.87,21,14z M17,14l2,0c0,2.76-2.24,5-5,5l0-2 C15.66,17,17,15.66,17,14z"/></g></g></svg>
            </svg>
        }
    }
}


