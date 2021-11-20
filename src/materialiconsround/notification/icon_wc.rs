
pub struct IconWc {
  props: crate::Props,
}

impl yew::Component for IconWc {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M.01 0h24v24h-24V0z" fill="none"/><path d="M5.5 21v-6.5H5c-.55 0-1-.45-1-1V9c0-1.1.9-2 2-2h3c1.1 0 2 .9 2 2v4.5c0 .55-.45 1-1 1h-.5V21c0 .55-.45 1-1 1h-2c-.55 0-1-.45-1-1zM18 21v-5h1.61c.68 0 1.16-.67.95-1.32l-2.1-6.31C18.18 7.55 17.42 7 16.56 7h-.12c-.86 0-1.63.55-1.9 1.37l-2.1 6.31c-.22.65.26 1.32.95 1.32H15v5c0 .55.45 1 1 1h1c.55 0 1-.45 1-1zM7.5 6c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2zm9 0c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2z"/></svg>
            </svg>
        }
    }
}


