
pub struct IconAtm {
  props: crate::Props,
}

impl yew::Component for IconAtm {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M3.5 13.5h2V15H7v-5c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v5h1.5v-1.5zm0-3h2V12h-2v-1.5zm13.5 0h1V14h1.5v-3.51h1V15H22v-5c0-.55-.45-1-1-1h-4.5c-.55 0-1 .45-1 1v5H17v-4.5zM10.25 15h1.5v-4.5H14V9H8v1.5h2.25z"/></svg>
            </svg>
        }
    }
}


