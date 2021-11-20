
pub struct IconFreeCancellation {
  props: crate::Props,
}

impl yew::Component for IconFreeCancellation {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M11.21,20H5V10h14v4.38l2-2V4h-3V2h-2v2H8V2H6v2H3v18h10.21L11.21,20z M16.54,22.5L13,18.96l1.41-1.41l2.12,2.12l4.24-4.24 l1.41,1.41L16.54,22.5z M10.41,14L12,15.59L10.59,17L9,15.41L7.41,17L6,15.59L7.59,14L6,12.41L7.41,11L9,12.59L10.59,11L12,12.41 L10.41,14z"/></svg>
            </svg>
        }
    }
}


