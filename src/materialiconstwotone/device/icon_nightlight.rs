
pub struct IconNightlight {
  props: crate::Props,
}

impl yew::Component for IconNightlight {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><g><g><path d="M6,12c0-4.41,3.59-8,8-8c0.34,0,0.68,0.02,1.01,0.07 C13.1,6.23,12,9.05,12,12s1.1,5.77,3.01,7.93C14.68,19.98,14.34,20,14,20C9.59,20,6,16.41,6,12z" enable-background="new" opacity=".3"/><path d="M14,12c0-3.7,2.01-6.92,5-8.65C17.53,2.5,15.82,2,14,2C8.48,2,4,6.48,4,12s4.48,10,10,10c1.82,0,3.53-0.5,5-1.35 C16.01,18.92,14,15.7,14,12z M15.01,19.93C14.68,19.98,14.34,20,14,20c-4.41,0-8-3.59-8-8s3.59-8,8-8 c0.34,0,0.68,0.02,1.01,0.07C13.1,6.23,12,9.05,12,12S13.1,17.77,15.01,19.93z"/></g></g></g></g></svg>
            </svg>
        }
    }
}


