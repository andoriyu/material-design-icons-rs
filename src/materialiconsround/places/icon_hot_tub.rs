
pub struct IconHotTub {
  props: crate::Props,
}

impl yew::Component for IconHotTub {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><circle cx="7" cy="6" r="2"/><path d="M21 12h-9.85c-.31-.22-.59-.46-.82-.72l-1.4-1.55c-.19-.21-.43-.38-.69-.5-.29-.14-.62-.23-.96-.23h-.03C6.01 9 5 10.01 5 11.25V12H3c-.55 0-1 .45-1 1v7c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-7c0-.55-.45-1-1-1zM7 19c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm4 0c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm4 0c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm4 0c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm-3.94-9c.5 0 .93-.39.94-.89.04-1.4-.58-2.48-1.35-3.25-.65-.72-.8-1.27-.77-1.91.02-.52-.41-.95-.94-.95-.5 0-.93.4-.94.9-.03 1.29.5 2.43 1.35 3.25.61.59.78 1.27.78 1.89-.01.52.4.96.93.96zm4 0c.5 0 .93-.39.94-.89.04-1.4-.58-2.48-1.35-3.25-.65-.72-.8-1.27-.77-1.91.02-.52-.41-.95-.94-.95-.5 0-.93.4-.94.9-.03 1.29.5 2.43 1.35 3.25.61.59.78 1.27.78 1.89-.01.52.4.96.93.96z"/></svg>
            </svg>
        }
    }
}


