
pub struct IconPhoneMissed {
  props: crate::Props,
}

impl yew::Component for IconPhoneMissed {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M23.09 16.2c-6.33-5.59-15.86-5.59-22.18 0-.84.74-.84 2.05-.05 2.84l1.2 1.2c.71.71 1.84.77 2.62.15l1.97-1.57c.47-.37.75-.94.75-1.55V14.7c2.98-.97 6.21-.98 9.2 0v2.58c0 .6.28 1.17.75 1.55l1.96 1.56c.79.62 1.91.56 2.62-.15l1.2-1.2c.8-.79.79-2.1-.04-2.84zM6 9c.55 0 1-.45 1-1V6.43l4.24 4.24c.39.39 1.02.39 1.41 0l5.66-5.66c.39-.39.39-1.02 0-1.41-.39-.39-1.02-.39-1.41 0l-4.95 4.95L8.4 5H10c.55 0 1-.45 1-1s-.45-1-1-1H6c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1z"/></svg>
            </svg>
        }
    }
}


