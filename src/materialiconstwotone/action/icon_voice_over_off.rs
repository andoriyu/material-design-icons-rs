
pub struct IconVoiceOverOff {
  props: crate::Props,
}

impl yew::Component for IconVoiceOverOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M9 17c-2.69 0-5.77 1.28-6 2h12c-.2-.71-3.3-2-6-2zM7 9c0 1.1.9 2 2 2 .22 0 .42-.04.62-.11L7.11 8.38c-.07.2-.11.4-.11.62z" opacity=".3"/><path d="M16.76 5.36l-1.68 1.69c.8 1.13.83 2.58.09 3.74l1.7 1.7c1.9-2.02 1.87-4.98-.11-7.13zM20.07 2l-1.63 1.63c2.72 2.97 2.76 7.39.14 10.56l1.64 1.64c3.74-3.89 3.71-9.84-.15-13.83zM9.43 5.04l3.53 3.53c-.2-1.86-1.67-3.33-3.53-3.53zM4.41 2.86L3 4.27l2.62 2.62C5.23 7.5 5 8.22 5 9c0 2.21 1.79 4 4 4 .78 0 1.5-.23 2.11-.62l4.4 4.4C13.74 15.6 10.78 15 9 15c-2.67 0-8 1.34-8 4v2h16v-2c0-.37-.11-.7-.29-1.02L19.73 21l1.41-1.41L4.41 2.86zM3 19c.22-.72 3.31-2 6-2 2.7 0 5.8 1.29 6 2H3zm6-8c-1.1 0-2-.9-2-2 0-.22.04-.42.11-.62l2.51 2.51c-.2.07-.4.11-.62.11z"/></svg>
            </svg>
        }
    }
}


