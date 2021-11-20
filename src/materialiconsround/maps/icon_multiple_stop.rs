
pub struct IconMultipleStop {
  props: crate::Props,
}

impl yew::Component for IconMultipleStop {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M17,5.21c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79C17.54,11.46,17,11.24,17,10.79V9h-3 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3V5.21z M10,7C9.45,7,9,7.45,9,8s0.45,1,1,1s1-0.45,1-1S10.55,7,10,7z M6,7 C5.45,7,5,7.45,5,8s0.45,1,1,1s1-0.45,1-1S6.55,7,6,7z M7,17h3c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7v-1.79 c0-0.45-0.54-0.67-0.85-0.35l-2.79,2.79c-0.2,0.2-0.2,0.51,0,0.71l2.79,2.79C6.46,19.46,7,19.24,7,18.79V17z M14,17 c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1s-1,0.45-1,1C13,16.55,13.45,17,14,17z M18,17c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1 s-1,0.45-1,1C17,16.55,17.45,17,18,17z"/></g></svg>
            </svg>
        }
    }
}


