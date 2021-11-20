
pub struct IconRestorePage {
  props: crate::Props,
}

impl yew::Component for IconRestorePage {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm4 18H6V4h7.17L18 8.83V20zm-9.55-9.43L7.28 9.4V13h3.6l-1.44-1.44c.52-1.01 1.58-1.71 2.79-1.71 1.74 0 3.15 1.41 3.15 3.15s-1.41 3.15-3.15 3.15c-1.07 0-2.02-.54-2.58-1.35H8.1c.69 1.58 2.28 2.7 4.12 2.7 2.48 0 4.5-2.02 4.5-4.5s-2.02-4.5-4.5-4.5c-1.59 0-2.97.83-3.77 2.07z"/></svg>
            </svg>
        }
    }
}


