
pub struct IconMp {
  props: crate::Props,
}

impl yew::Component for IconMp {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M7,9h4.5c0.55,0,1,0.45,1,1v4.25 c0,0.41-0.34,0.75-0.75,0.75h0C11.34,15,11,14.66,11,14.25V10.5h-1v2.25c0,0.41-0.34,0.75-0.75,0.75h0 c-0.41,0-0.75-0.34-0.75-0.75V10.5h-1v3.75C7.5,14.66,7.16,15,6.75,15h0C6.34,15,6,14.66,6,14.25V10C6,9.45,6.45,9,7,9z M14.25,15 L14.25,15c-0.41,0-0.75-0.33-0.75-0.75V10c0-0.55,0.45-1,1-1H17c0.55,0,1,0.45,1,1v2.5c0,0.55-0.45,1-1,1h-2v0.75 C15,14.66,14.66,15,14.25,15z M15,12h1.5v-1.5H15V12z"/></g></g></svg>
            </svg>
        }
    }
}


