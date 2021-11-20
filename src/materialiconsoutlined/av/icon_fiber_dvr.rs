
pub struct IconFiberDvr {
  props: crate::Props,
}

impl yew::Component for IconFiberDvr {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M11.87 12.43l-1-3.43h-1.5l1.75 6h1.5l1.75-6h-1.5zM21 11.5v-1c0-.85-.65-1.5-1.5-1.5H16v6h1.5v-2h1.15l.85 2H21l-.9-2.1c.5-.25.9-.8.9-1.4zm-1.5 0h-2v-1h2v1zM6.5 9H3v6h3.5c.85 0 1.5-.65 1.5-1.5v-3C8 9.65 7.35 9 6.5 9zm0 4.5h-2v-3h2v3z"/></svg>
            </svg>
        }
    }
}


