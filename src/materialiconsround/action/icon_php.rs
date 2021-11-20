
pub struct IconPhp {
  props: crate::Props,
}

impl yew::Component for IconPhp {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M6.5,10.5h-2v1h2V10.5z M20,10.5h-2v1h2V10.5z M13,12.5h-2v1.75c0,0.41-0.34,0.75-0.75,0.75S9.5,14.66,9.5,14.25v-4.5 C9.5,9.34,9.84,9,10.25,9S11,9.34,11,9.75V11h2V9.75C13,9.34,13.34,9,13.75,9s0.75,0.34,0.75,0.75v4.5c0,0.41-0.34,0.75-0.75,0.75 S13,14.66,13,14.25V12.5z M18,14.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75V10c0-0.55,0.45-1,1-1H20 c0.83,0,1.5,0.68,1.5,1.5v1c0,0.82-0.67,1.5-1.5,1.5h-2V14.25z M3,10c0-0.55,0.45-1,1-1h2.5C7.33,9,8,9.68,8,10.5v1 C8,12.32,7.33,13,6.5,13h-2v1.25C4.5,14.66,4.16,15,3.75,15S3,14.66,3,14.25V10z"/></g></svg>
            </svg>
        }
    }
}


