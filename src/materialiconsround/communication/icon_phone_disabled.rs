
pub struct IconPhoneDisabled {
  props: crate::Props,
}

impl yew::Component for IconPhoneDisabled {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><g><g><path d="M14.54,17.37c-2.63,2.08-5.89,3.39-9.45,3.61c-1.13,0.07-2.07-0.87-2.07-2v-1.73 c-0.01-1.01,0.75-1.86,1.76-1.98l2.54-0.29c0.61-0.07,1.21,0.14,1.64,0.57l1.84,1.84c0.81-0.41,1.59-0.9,2.31-1.45L2.1,4.93 c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.03-0.39,1.42,0L20.49,20.5c0.39,0.39,0.39,1.02,0,1.41s-1.02,0.39-1.41,0 L14.54,17.37z M17.39,10.8l-1.85-1.85c-0.43-0.43-0.64-1.03-0.57-1.64l0.29-2.52c0.12-1.01,0.97-1.77,1.99-1.77h1.73 c1.13,0,2.07,0.94,2,2.07c-0.22,3.57-1.54,6.83-3.62,9.47l-1.43-1.43C16.48,12.4,16.97,11.62,17.39,10.8z"/></g></g></svg>
            </svg>
        }
    }
}


