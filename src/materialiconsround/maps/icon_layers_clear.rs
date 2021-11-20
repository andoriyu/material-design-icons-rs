
pub struct IconLayersClear {
  props: crate::Props,
}

impl yew::Component for IconLayersClear {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M19.99 9.79c.51-.4.51-1.18 0-1.58l-6.76-5.26c-.72-.56-1.73-.56-2.46 0L9.41 4.02l7.88 7.88 2.7-2.11zm0 3.49l-.01-.01c-.36-.28-.86-.28-1.22 0l-.05.04 1.4 1.4c.37-.41.34-1.07-.12-1.43zm1.45 5.6L4.12 1.56c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l3.52 3.52-2.22 1.72c-.51.4-.51 1.18 0 1.58l6.76 5.26c.72.56 1.73.56 2.46 0l.87-.68 1.42 1.42-2.92 2.27c-.36.28-.87.28-1.23 0l-6.15-4.78c-.36-.28-.86-.28-1.22 0-.51.4-.51 1.17 0 1.57l6.76 5.26c.72.56 1.73.56 2.46 0l3.72-2.89 3.07 3.07c.39.39 1.02.39 1.41 0 .41-.39.41-1.02.02-1.41z"/></svg>
            </svg>
        }
    }
}


