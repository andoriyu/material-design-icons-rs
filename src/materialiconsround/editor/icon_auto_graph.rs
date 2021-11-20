
pub struct IconAutoGraph {
  props: crate::Props,
}

impl yew::Component for IconAutoGraph {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><g><path d="M14.06,9.94L13,9.45c-0.39-0.18-0.39-0.73,0-0.91l1.06-0.49L14.55,7c0.18-0.39,0.73-0.39,0.91,0l0.49,1.06L17,8.55 c0.39,0.18,0.39,0.73,0,0.91l-1.06,0.49L15.45,11c-0.18,0.39-0.73,0.39-0.91,0L14.06,9.94z M4.45,13l0.49-1.06L6,11.45 c0.39-0.18,0.39-0.73,0-0.91l-1.06-0.49L4.45,9C4.28,8.61,3.72,8.61,3.55,9l-0.49,1.06L2,10.55c-0.39,0.18-0.39,0.73,0,0.91 l1.06,0.49L3.55,13C3.72,13.39,4.28,13.39,4.45,13z M8.96,7.99l0.63-1.4l1.4-0.63c0.39-0.18,0.39-0.73,0-0.91l-1.4-0.63l-0.63-1.4 c-0.18-0.39-0.73-0.39-0.91,0l-0.63,1.4l-1.4,0.63c-0.39,0.18-0.39,0.73,0,0.91l1.4,0.63l0.63,1.4C8.22,8.38,8.78,8.38,8.96,7.99z M22.34,8.27c-0.4-0.4-1.07-0.39-1.45,0.04l-6.39,7.18l-3.29-3.29c-0.39-0.39-1.02-0.39-1.41,0l-6.04,6.05 c-0.41,0.41-0.41,1.09,0,1.5c0.41,0.41,1.09,0.41,1.5,0l5.25-5.26l3.25,3.25c0.41,0.41,1.07,0.39,1.45-0.04l7.17-8.07 C22.73,9.24,22.71,8.64,22.34,8.27z"/></g></svg>
            </svg>
        }
    }
}

