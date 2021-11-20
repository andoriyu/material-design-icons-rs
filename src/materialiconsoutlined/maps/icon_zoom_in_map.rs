
pub struct IconZoomInMap {
  props: crate::Props,
}

impl yew::Component for IconZoomInMap {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M9,9l0-6L7,3l0,2.59L3.91,2.5L2.5,3.91L5.59,7L3,7l0,2L9,9z M21,9V7l-2.59,0l3.09-3.09L20.09,2.5L17,5.59V3l-2,0l0,6L21,9z M3,15l0,2h2.59L2.5,20.09l1.41,1.41L7,18.41L7,21h2l0-6L3,15z M15,15l0,6h2v-2.59l3.09,3.09l1.41-1.41L18.41,17H21v-2L15,15z"/></g></svg>
            </svg>
        }
    }
}


