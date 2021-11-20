
pub struct IconTextRotateUp {
  props: crate::Props,
}

impl yew::Component for IconTextRotateUp {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0zm0 0h24v24H0V0zm0 0h24v24H0V0z" fill="none"/><path d="M18.35 4.35c-.2-.2-.51-.2-.71 0l-1.79 1.79c-.31.32-.09.86.36.86H17v12c0 .55.45 1 1 1s1-.45 1-1V7h.79c.45 0 .67-.54.35-.85l-1.79-1.8zM11.8 15.5v-5l1.6-.66c.36-.14.6-.49.6-.88 0-.69-.71-1.15-1.34-.88l-8.97 3.88c-.42.17-.69.58-.69 1.04 0 .46.27.87.69 1.05l8.97 3.88c.63.27 1.34-.2 1.34-.89 0-.39-.24-.74-.6-.89l-1.6-.65zM4.98 13L10 11.13v3.74L4.98 13z"/></svg>
            </svg>
        }
    }
}


