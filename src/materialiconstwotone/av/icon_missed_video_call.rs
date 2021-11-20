
pub struct IconMissedVideoCall {
  props: crate::Props,
}

impl yew::Component for IconMissedVideoCall {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M15 13.5V8H5v8h10v-2.5zM11 15l-3.89-3.89v2.55H6V9.22h4.44v1.11H7.89l3.11 3.1 2.99-3.01.78.79L11 15z" opacity=".3"/><path d="M3 17c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-3.5l4 4v-11l-4 4V7c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v10zm2-9h10v8H5V8zm6 5.43l-3.11-3.1h2.55V9.22H6v4.44h1.11v-2.55L11 15l3.77-3.79-.78-.79z"/></svg>
            </svg>
        }
    }
}


