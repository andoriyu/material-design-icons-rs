
pub struct IconFilterListOff {
  props: crate::Props,
}

impl yew::Component for IconFilterListOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M21,7c0-0.55-0.45-1-1-1H8.83l2,2H20C20.55,8,21,7.55,21,7z M18,12c0-0.55-0.45-1-1-1h-3.17l2,2H17 C17.55,13,18,12.55,18,12z M13.98,16.81C13.99,16.87,14,16.94,14,17c0,0.55-0.45,1-1,1h-2c-0.55,0-1-0.45-1-1s0.45-1,1-1h2 c0.06,0,0.13,0.01,0.19,0.02L10.17,13H7c-0.55,0-1-0.45-1-1s0.45-1,1-1h1.17l-3-3H4C3.45,8,3,7.55,3,7c0-0.32,0.15-0.6,0.38-0.79 L2.1,4.93c-0.39-0.39-0.39-1.02,0-1.41s1.02-0.39,1.41,0l16.97,16.97c0.39,0.39,0.39,1.02,0,1.41s-1.02,0.39-1.41,0L13.98,16.81z"/></g></svg>
            </svg>
        }
    }
}


