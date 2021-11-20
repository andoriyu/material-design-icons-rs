
pub struct IconSunnySnowing {
  props: crate::Props,
}

impl yew::Component for IconSunnySnowing {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M11,4V2c0-0.55,0.45-1,1-1s1,0.45,1,1v2c0,0.55-0.45,1-1,1S11,4.55,11,4z M18.36,7.05l1.41-1.42c0.39-0.39,0.39-1.02,0-1.41 c-0.39-0.39-1.02-0.39-1.41,0l-1.41,1.42c-0.39,0.39-0.39,1.02,0,1.41C17.34,7.44,17.97,7.44,18.36,7.05z M22,11h-2 c-0.55,0-1,0.45-1,1s0.45,1,1,1h2c0.55,0,1-0.45,1-1S22.55,11,22,11z M5.64,7.05L4.22,5.64c-0.39-0.39-0.39-1.03,0-1.41 s1.03-0.39,1.41,0l1.41,1.41c0.39,0.39,0.39,1.03,0,1.41S6.02,7.44,5.64,7.05z M2,13h2c0.55,0,1-0.45,1-1s-0.45-1-1-1H2 c-0.55,0-1,0.45-1,1S1.45,13,2,13z M7,13.5c0.55,0,9.45,0,10,0c0.55,0,1-0.45,1-1v-0.29c0-3.05-2.19-5.77-5.21-6.16 C9.13,5.58,6,8.43,6,12v0.5C6,13.05,6.45,13.5,7,13.5z M5,18c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S5,17.45,5,18z M17,18 c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S17,17.45,17,18z M8,22c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S8,21.45,8,22z M11,18 c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S11,17.45,11,18z M14,22c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S14,21.45,14,22z"/></svg>
            </svg>
        }
    }
}


