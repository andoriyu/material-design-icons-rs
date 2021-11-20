
pub struct IconSettingsSuggest {
  props: crate::Props,
}

impl yew::Component for IconSettingsSuggest {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><g><path d="M18.04,7.99l-0.63-1.4l-1.4-0.63c-0.39-0.18-0.39-0.73,0-0.91l1.4-0.63l0.63-1.4c0.18-0.39,0.73-0.39,0.91,0l0.63,1.4 l1.4,0.63c0.39,0.18,0.39,0.73,0,0.91l-1.4,0.63l-0.63,1.4C18.78,8.38,18.22,8.38,18.04,7.99z M21.28,12.72L20.96,12 c-0.18-0.39-0.73-0.39-0.91,0l-0.32,0.72L19,13.04c-0.39,0.18-0.39,0.73,0,0.91l0.72,0.32L20.04,15c0.18,0.39,0.73,0.39,0.91,0 l0.32-0.72L22,13.96c0.39-0.18,0.39-0.73,0-0.91L21.28,12.72z M16.24,14.37l1.23,0.93c0.4,0.3,0.51,0.86,0.26,1.3l-1.62,2.8 c-0.25,0.44-0.79,0.62-1.25,0.42l-1.43-0.6c-0.2,0.13-0.42,0.26-0.64,0.37l-0.19,1.54c-0.06,0.5-0.49,0.88-0.99,0.88H8.38 c-0.5,0-0.93-0.38-0.99-0.88L7.2,19.59c-0.22-0.11-0.43-0.23-0.64-0.37l-1.43,0.6c-0.46,0.2-1,0.02-1.25-0.42l-1.62-2.8 c-0.25-0.44-0.14-0.99,0.26-1.3l1.23-0.93C3.75,14.25,3.75,14.12,3.75,14s0-0.25,0.01-0.37L2.53,12.7c-0.4-0.3-0.51-0.86-0.26-1.3 l1.62-2.8c0.25-0.44,0.79-0.62,1.25-0.42l1.43,0.6c0.2-0.13,0.42-0.26,0.64-0.37l0.19-1.54C7.45,6.38,7.88,6,8.38,6h3.23 c0.5,0,0.93,0.38,0.99,0.88l0.19,1.54c0.22,0.11,0.43,0.23,0.64,0.37l1.43-0.6c0.46-0.2,1-0.02,1.25,0.42l1.62,2.8 c0.25,0.44,0.14,0.99-0.26,1.3l-1.23,0.93c0.01,0.12,0.01,0.24,0.01,0.37S16.25,14.25,16.24,14.37z M13,14c0-1.66-1.34-3-3-3 s-3,1.34-3,3s1.34,3,3,3S13,15.66,13,14z"/></g></svg>
            </svg>
        }
    }
}

