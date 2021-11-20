
pub struct IconDoNotTouch {
  props: crate::Props,
}

impl yew::Component for IconDoNotTouch {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M13,10.17l-2.5-2.5V1H13V10.17z M20,4h-2.5v7h-1V2H14v9.17l6,6V4z M9.5,3H7.01v1.18L9.5,6.67V3z M21.19,21.19L2.81,2.81 L1.39,4.22L7,9.83v4.3l-3.32-1.9L2,13.88L9.68,22h9.54l0.56,0.61L21.19,21.19z"/></g></svg>
            </svg>
        }
    }
}


