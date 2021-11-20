
pub struct IconNoLuggage {
  props: crate::Props,
}

impl yew::Component for IconNoLuggage {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M12.75,9v0.92l1.75,1.75V9H16v4.17l3,3V6h-4V2H9v4H8.83l3,3H12.75z M10.5,3.5h3V6h-3V3.5z M21.19,21.19L2.81,2.81L1.39,4.22 L5,7.83V21h2v1h2v-1h6v1h2v-1h1.17l1.61,1.61L21.19,21.19z M8,18v-7.17l1.5,1.5V18H8z M11.25,18v-3.92l1.5,1.5V18H11.25z"/></svg>
            </svg>
        }
    }
}


