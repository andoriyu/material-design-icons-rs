
pub struct IconHtml {
  props: crate::Props,
}

impl yew::Component for IconHtml {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M21,15c-0.55,0-1-0.45-1-1V9.75C20,9.34,20.34,9,20.75,9s0.75,0.34,0.75,0.75v3.75h1.75c0.41,0,0.75,0.34,0.75,0.75 c0,0.41-0.34,0.75-0.75,0.75H21z M16,10.49h1v3.76c0,0.41,0.34,0.75,0.75,0.75s0.75-0.34,0.75-0.75V10c0-0.55-0.45-1-1-1H13 c-0.55,0-1,0.45-1,1v4.25c0,0.41,0.34,0.75,0.75,0.75s0.75-0.34,0.75-0.75V10.5h1v2.75c0,0.41,0.34,0.75,0.75,0.75 S16,13.66,16,13.25V10.49z M5,9.75C5,9.34,4.66,9,4.25,9S3.5,9.34,3.5,9.75V11h-2V9.75C1.5,9.34,1.16,9,0.75,9S0,9.34,0,9.75v4.5 C0,14.66,0.34,15,0.75,15s0.75-0.34,0.75-0.75V12.5h2v1.75C3.5,14.66,3.84,15,4.25,15S5,14.66,5,14.25V9.75z M10.25,10.5 c0.41,0,0.75-0.34,0.75-0.75C11,9.34,10.66,9,10.25,9h-3.5C6.34,9,6,9.34,6,9.75c0,0.41,0.34,0.75,0.75,0.75h1v3.75 C7.75,14.66,8.09,15,8.5,15s0.75-0.34,0.75-0.75V10.5H10.25z"/></g></svg>
            </svg>
        }
    }
}


