
pub struct IconExposureZero {
  props: crate::Props,
}

impl yew::Component for IconExposureZero {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M15.01 6.22c-.36-.43-.8-.74-1.31-.93S12.63 5 12 5c-.62 0-1.19.1-1.69.29-.51.19-.95.5-1.31.93s-.64.99-.84 1.69c-.2.7-.3 1.55-.3 2.55v2.04c0 1 .1 1.85.3 2.55.2.69.49 1.26.85 1.7s.8.75 1.31.95c.51.2 1.07.3 1.69.3.63 0 1.19-.1 1.7-.3.51-.2.94-.51 1.3-.95.35-.43.63-1 .83-1.7.2-.7.3-1.55.3-2.55h.01v-2.04c0-1.01-.1-1.85-.3-2.55-.2-.7-.48-1.26-.84-1.69zm-.97 6.58c0 .63-.04 1.17-.13 1.62-.09.45-.22.81-.39 1.09s-.38.48-.63.61-.54.19-.87.19c-.33 0-.62-.06-.87-.19s-.47-.33-.64-.61c-.17-.28-.31-.64-.4-1.09-.09-.44-.14-.98-.14-1.62v-2.67c0-.64.04-1.18.13-1.62.09-.44.23-.79.4-1.06s.39-.46.64-.58.54-.18.86-.18.61.06.86.19c.25.12.47.31.64.58.18.27.31.62.4 1.06.08.44.13.98.13 1.62h.01v2.66z"/></svg>
            </svg>
        }
    }
}


