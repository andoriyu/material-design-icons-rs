
pub struct IconAssuredWorkload {
  props: crate::Props,
}

impl yew::Component for IconAssuredWorkload {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M6,17c0.55,0,1-0.45,1-1v-5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v5C5,16.55,5.45,17,6,17L6,17z"/><path d="M12,17c0.55,0,1-0.45,1-1v-5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v5C11,16.55,11.45,17,12,17L12,17z"/><path d="M21.32,5.66l-8.42-4.21c-0.56-0.28-1.23-0.28-1.79,0L2.68,5.66C2.26,5.87,2,6.3,2,6.76v0C2,7.45,2.55,8,3.24,8h17.53 C21.45,8,22,7.45,22,6.76v0C22,6.3,21.74,5.87,21.32,5.66z"/><path d="M2,20L2,20c0,0.55,0.45,1,1,1h11.4c-0.21-0.64-0.32-1.31-0.36-2H3C2.45,19,2,19.45,2,20z"/><path d="M19,12.26V11c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2.26L19,12.26z"/><path d="M19.55,14.22l-3,1.5C16.21,15.89,16,16.24,16,16.62v1.93c0,2.52,1.71,4.88,4,5.45c2.29-0.57,4-2.93,4-5.45v-1.93 c0-0.38-0.21-0.73-0.55-0.89l-3-1.5C20.17,14.08,19.83,14.08,19.55,14.22z M18.58,20.3l-0.8-0.8c-0.29-0.29-0.29-0.77,0-1.06l0,0 c0.29-0.29,0.77-0.29,1.06,0l0.44,0.44l1.88-1.85c0.29-0.29,0.77-0.29,1.06,0l0,0c0.29,0.29,0.29,0.77,0,1.06l-2.23,2.21 C19.6,20.69,18.97,20.69,18.58,20.3z"/></g></g></svg>
            </svg>
        }
    }
}


