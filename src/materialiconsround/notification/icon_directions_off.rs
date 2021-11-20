
pub struct IconDirectionsOff {
  props: crate::Props,
}

impl yew::Component for IconDirectionsOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M21.71,11.29l-9-9c-0.39-0.39-1.02-0.39-1.41,0L8.21,5.38L13,10.17l0-2.67l3.15,3.15c0.2,0.2,0.2,0.51,0,0.71l-0.98,0.98 l3.45,3.45l3.09-3.09C22.09,12.32,22.09,11.69,21.71,11.29z"/></g><g><path d="M6.79,6.79L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l3.28,3.28l-3.09,3.09 c-0.39,0.39-0.39,1.02,0,1.41l9,9c0.39,0.39,1.02,0.39,1.41,0l3.09-3.09l3.28,3.28c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L6.79,6.79z M9.99,14c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1v-3c0-0.05,0.02-0.1,0.03-0.15l1.97,1.97 V14z"/></g></g></g></svg>
            </svg>
        }
    }
}


