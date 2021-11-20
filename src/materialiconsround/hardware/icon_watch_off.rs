
pub struct IconWatchOff {
  props: crate::Props,
}

impl yew::Component for IconWatchOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M12,7c2.76,0,5,2.24,5,5c0,0.64-0.13,1.25-0.35,1.82l1.5,1.5C18.69,14.33,19,13.2,19,12c0-2.22-1.03-4.19-2.64-5.47 l-0.93-3.1C15.17,2.58,14.4,2,13.51,2h-3.02C9.6,2,8.83,2.58,8.57,3.42L8.04,5.21l2.14,2.14C10.75,7.13,11.36,7,12,7z"/><path d="M2.1,3.51L2.1,3.51c-0.39,0.39-0.39,1.02,0,1.41l3.75,3.75C5.31,9.67,5,10.8,5,12c0,2.22,1.03,4.19,2.64,5.47l0.93,3.1 C8.83,21.42,9.6,22,10.49,22h3.02c0.88,0,1.66-0.58,1.92-1.43l0.53-1.78l3.11,3.11c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51C3.12,3.12,2.49,3.12,2.1,3.51z M12,17c-2.76,0-5-2.24-5-5c0-0.64,0.13-1.25,0.35-1.82 l6.47,6.47C13.25,16.87,12.64,17,12,17z"/></g></g></svg>
            </svg>
        }
    }
}


