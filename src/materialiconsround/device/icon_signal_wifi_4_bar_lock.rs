
pub struct IconSignalWifi4BarLock {
  props: crate::Props,
}

impl yew::Component for IconSignalWifi4BarLock {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M23.21,8.24C20.22,5.6,16.3,4,12,4S3.78,5.6,0.79,8.24C0.35,8.63,0.32,9.3,0.73,9.71l5.62,5.63l4.94,4.95 c0.39,0.39,1.02,0.39,1.42,0l2.34-2.34V15c0-0.45,0.09-0.88,0.23-1.29c0.54-1.57,2.01-2.71,3.77-2.71h2.94l1.29-1.29 C23.68,9.3,23.65,8.63,23.21,8.24z"/><path d="M22,16v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3 C23,16.45,22.55,16,22,16z M21,16h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V16z"/></g></g></svg>
            </svg>
        }
    }
}


