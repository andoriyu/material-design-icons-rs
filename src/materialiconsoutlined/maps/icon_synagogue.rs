
pub struct IconSynagogue {
  props: crate::Props,
}

impl yew::Component for IconSynagogue {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M20,4c-1.66,0-3,1.34-3,3v0.29L12,3L7,7.29V7c0-1.66-1.34-3-3-3S1,5.34,1,7v14h10v-5c0-0.55,0.45-1,1-1s1,0.45,1,1v5h10V7 C23,5.34,21.66,4,20,4z M20,6c0.55,0,1,0.45,1,1v1h-2V7C19,6.45,19.45,6,20,6z M4,6c0.55,0,1,0.45,1,1v1H3V7C3,6.45,3.45,6,4,6z M3,19v-9h2v9H3z M17,19h-2v-3c0-1.65-1.35-3-3-3c-1.65,0-3,1.35-3,3v3H7V9.92l5-4.29l5,4.29V19z M19,19v-9h2v9H19z"/><circle cx="12" cy="10" r="1.5"/></g></g></svg>
            </svg>
        }
    }
}


