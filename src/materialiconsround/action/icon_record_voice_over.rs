
pub struct IconRecordVoiceOver {
  props: crate::Props,
}

impl yew::Component for IconRecordVoiceOver {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><circle cx="9" cy="9" r="4"/><path d="M9 15c-2.67 0-8 1.34-8 4v1c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-1c0-2.66-5.33-4-8-4zm6.47-7.23c.32.79.32 1.67 0 2.46-.19.47-.11 1 .25 1.36l.03.03c.58.58 1.57.46 1.95-.27.76-1.45.76-3.15-.02-4.66-.38-.74-1.38-.88-1.97-.29l-.01.01c-.34.35-.42.89-.23 1.36zm3.71-4.88c-.4.4-.46 1.02-.13 1.48 1.97 2.74 1.96 6.41-.03 9.25-.32.45-.25 1.07.14 1.46l.03.03c.49.49 1.32.45 1.74-.1 2.75-3.54 2.76-8.37 0-12.02-.42-.55-1.26-.59-1.75-.1z"/></svg>
            </svg>
        }
    }
}


