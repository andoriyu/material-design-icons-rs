
pub struct IconStream {
  props: crate::Props,
}

impl yew::Component for IconStream {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><circle cx="20" cy="12" r="2"/><circle cx="4" cy="12" r="2"/><circle cx="12" cy="20" r="2"/><path d="M7.89,14.65l-2.94,2.93c-0.39,0.39-0.39,1.02,0,1.41s1.02,0.39,1.41,0l2.94-2.93c0.39-0.38,0.39-1.02,0-1.41 C8.91,14.26,8.28,14.26,7.89,14.65z"/><path d="M6.41,4.94C6.02,4.55,5.39,4.55,5,4.94C4.61,5.33,4.61,5.96,5,6.35l2.93,2.94c0.39,0.39,1.02,0.39,1.42,0 C9.73,8.9,9.73,8.27,9.34,7.88L6.41,4.94z"/><path d="M16.12,14.65c-0.39-0.39-1.02-0.39-1.42,0c-0.39,0.39-0.39,1.02,0,1.41L17.64,19c0.39,0.39,1.02,0.39,1.41,0 s0.39-1.02,0-1.41L16.12,14.65z"/><path d="M16.06,9.33l2.99-2.98c0.39-0.4,0.39-1.03,0-1.42c-0.39-0.39-1.02-0.39-1.41,0l-2.99,2.98c-0.39,0.39-0.39,1.02,0,1.42 C15.04,9.72,15.67,9.72,16.06,9.33z"/><circle cx="12" cy="4" r="2"/></g></g></svg>
            </svg>
        }
    }
}


