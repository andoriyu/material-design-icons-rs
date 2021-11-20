
pub struct IconWbIridescent {
  props: crate::Props,
}

impl yew::Component for IconWbIridescent {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M6 15h12c.55 0 1-.45 1-1v-3.95c0-.55-.45-1-1-1H6c-.55 0-1 .45-1 1V14c0 .55.45 1 1 1zm5-13v1.05c0 .55.45.95 1 .95s1-.4 1-.95V2c0-.55-.45-1-1-1s-1 .45-1 1zm7.34 2.3l-.38.38c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0l.38-.38c.39-.39.39-1.02 0-1.41-.39-.39-1.02-.39-1.41 0zM13 22v-.96c0-.55-.45-1-1-1s-1 .45-1 1V22c0 .55.45 1 1 1s1-.45 1-1zm6.74-3.61l-.39-.39c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l.38.39c.39.39 1.02.39 1.41 0l.01-.01c.39-.38.39-1.02 0-1.4zM4.25 5.71l.39.39c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41l-.39-.39c-.39-.39-1.02-.39-1.41 0-.38.39-.38 1.03 0 1.41zm1.42 14.08l.38-.38c.39-.39.39-1.02 0-1.41-.39-.39-1.02-.39-1.41 0l-.38.38c-.39.39-.39 1.02 0 1.41.38.39 1.02.39 1.41 0z"/></svg>
            </svg>
        }
    }
}


