
pub struct IconFort {
  props: crate::Props,
}

impl yew::Component for IconFort {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M19,8.17L20.17,7h-4.34L17,8.17V12H7V8.17L8.17,7H3.83L5,8.17v7.66l-2,2V19h5v-1c0-2.21,1.79-4,4-4 s4,1.79,4,4v1h5v-1.17l-2-2V8.17z" opacity=".3"/><path d="M23,7V3h-2v2h-2V3h-2v2h-2V3h-2v4l2,2v1H9V9l2-2V3H9v2H7V3H5v2H3V3H1v4l2,2v6l-2,2v4h9v-3c0-1.1,0.9-2,2-2s2,0.9,2,2v3h9 v-4l-2-2V9L23,7z M21,19h-5v-1c0-2.21-1.79-4-4-4s-4,1.79-4,4v1H3v-1.17l2-2V8.17L3.83,7h4.34L7,8.17V12h10V8.17L15.83,7h4.34 L19,8.17v7.66l2,2V19z"/></g></g></svg>
            </svg>
        }
    }
}

