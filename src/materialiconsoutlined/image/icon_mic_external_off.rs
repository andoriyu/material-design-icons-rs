
pub struct IconMicExternalOff {
  props: crate::Props,
}

impl yew::Component for IconMicExternalOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M10,5c0-1.66-1.34-3-3-3C6.38,2,5.81,2.19,5.33,2.5l4.15,4.15C9.8,6.18,10,5.61,10,5z"/><path d="M14,6c0-1.1,0.9-2,2-2s2,0.9,2,2v9.17l2,2V6c0-2.21-1.79-4-4-4s-4,1.79-4,4v3.17l2,2V6z"/><path d="M2.1,2.1L0.69,3.51L5.17,8H4l1,10h1c0,2.21,1.79,4,4,4s4-1.79,4-4v-1.17l6.49,6.49l1.41-1.41L2.1,2.1z M7.19,16H6.81 l-0.6-6h0.96l0.56,0.56L7.19,16z M12,18c0,1.1-0.9,2-2,2s-2-0.9-2-2h1l0.56-5.61L12,14.83V18z"/></g></g></svg>
            </svg>
        }
    }
}


