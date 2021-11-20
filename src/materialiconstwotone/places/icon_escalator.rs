
pub struct IconEscalator {
  props: crate::Props,
}

impl yew::Component for IconEscalator {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M19,5L19,5l0,14H5V5H19 M17,6h-3.3l-5,9H7c-0.83,0-1.5,0.67-1.5,1.5S6.17,18,7,18h3.3l5-9H17 c0.83,0,1.5-0.67,1.5-1.5S17.83,6,17,6z" opacity=".3"/><path d="M19,5L19,5l0,14H5V5H19 M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2l0-14C21,3.9,20.1,3,19,3L19,3z M17,6h-3.3l-5,9H7c-0.83,0-1.5,0.67-1.5,1.5S6.17,18,7,18h3.3l5-9H17c0.83,0,1.5-0.67,1.5-1.5S17.83,6,17,6z"/></g></svg>
            </svg>
        }
    }
}


