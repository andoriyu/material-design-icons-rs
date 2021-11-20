
pub struct IconAbc {
  props: crate::Props,
}

impl yew::Component for IconAbc {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M7.25,15c-0.41,0-0.75-0.34-0.75-0.75V13.5h-2v0.75C4.5,14.66,4.16,15,3.75,15S3,14.66,3,14.25V10c0-0.55,0.45-1,1-1h3 c0.55,0,1,0.45,1,1v4.25C8,14.66,7.66,15,7.25,15z M6.5,10.5h-2V12h2V10.5z M13.5,12c0.55,0,1,0.45,1,1v1c0,0.55-0.45,1-1,1h-3 c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v1C14.5,11.55,14.05,12,13.5,12z M11,10.5v0.75h2V10.5H11z M13,12.75 h-2v0.75h2V12.75z M21,10.25c0,0.41-0.34,0.75-0.75,0.75c-0.33,0-0.6-0.21-0.71-0.5l-2.04,0v3l2.04,0c0.1-0.29,0.38-0.5,0.71-0.5 c0.41,0,0.75,0.34,0.75,0.75V14c0,0.55-0.45,1-1,1h-3c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V10.25z"/></g></svg>
            </svg>
        }
    }
}


