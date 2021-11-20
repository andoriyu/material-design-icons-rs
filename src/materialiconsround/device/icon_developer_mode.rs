
pub struct IconDeveloperMode {
  props: crate::Props,
}

impl yew::Component for IconDeveloperMode {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M7 5h10v1c0 .55.45 1 1 1s1-.45 1-1V3c0-1.1-.9-1.99-2-1.99L7 1c-1.1 0-2 .9-2 2v3c0 .55.45 1 1 1s1-.45 1-1V5zm9.12 10.88l3.17-3.17c.39-.39.39-1.02 0-1.41l-3.17-3.17c-.39-.39-1.03-.39-1.42 0-.39.39-.39 1.02 0 1.41L17.17 12l-2.47 2.47c-.39.39-.39 1.02 0 1.41.39.39 1.03.39 1.42 0zm-6.83-1.42L6.83 12l2.46-2.46c.39-.39.39-1.02 0-1.41-.39-.39-1.03-.39-1.42 0L4.7 11.3c-.39.39-.39 1.02 0 1.41l3.17 3.17c.39.39 1.03.39 1.42 0 .4-.39.39-1.03 0-1.42zM17 19H7v-1c0-.55-.45-1-1-1s-1 .45-1 1v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2v-3c0-.55-.45-1-1-1s-1 .45-1 1v1z"/></svg>
            </svg>
        }
    }
}


