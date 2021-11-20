
pub struct IconAddToHomeScreen {
  props: crate::Props,
}

impl yew::Component for IconAddToHomeScreen {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M18 1.01L8 1c-1.1 0-2 .9-2 2v3c0 .55.45 1 1 1s1-.45 1-1V5h10v14H8v-1c0-.55-.45-1-1-1s-1 .45-1 1v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM11 15c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1H6c-.55 0-1 .45-1 1s.45 1 1 1h2.59L3.7 14.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L10 11.41V14c0 .55.45 1 1 1z"/></svg>
            </svg>
        }
    }
}


