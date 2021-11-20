
pub struct IconDomainVerification {
  props: crate::Props,
}

impl yew::Component for IconDomainVerification {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M10.23,15.83c0.39,0.39,1.02,0.39,1.41,0l4.24-4.24c0.39-0.39,0.39-1.02,0-1.42v0c-0.39-0.39-1.02-0.39-1.41,0l-3.54,3.53 l-1.41-1.41c-0.39-0.39-1.02-0.39-1.42,0s-0.39,1.02,0,1.41L10.23,15.83z"/><path d="M19,4H5C3.89,4,3,4.9,3,6v12c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6C21,4.9,20.11,4,19,4z M19,17c0,0.55-0.45,1-1,1H6 c-0.55,0-1-0.45-1-1V8h14V17z"/></g></g></svg>
            </svg>
        }
    }
}


