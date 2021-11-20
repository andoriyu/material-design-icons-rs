
pub struct IconCreditCardOff {
  props: crate::Props,
}

impl yew::Component for IconCreditCardOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M20,17.17V12h-5.17L20,17.17z M10.83,8H20V6H8.83L10.83,8z M4,6.83V8h1.17 L4,6.83z M15.17,18l-6-6H4v6H15.17z" enable-background="new" opacity=".3"/><path d="M6.83,4H20c1.11,0,2,0.89,2,2v12c0,0.34-0.08,0.66-0.23,0.94L20,17.17V12h-5.17l-4-4H20V6H8.83 L6.83,4z M20.49,23.31L17.17,20H4c-1.11,0-2-0.89-2-2L2.01,6c0-0.34,0.08-0.66,0.23-0.93L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M4,6.83V8h1.17L4,6.83z M15.17,18l-6-6H4v6H15.17z" enable-background="new"/></svg>
            </svg>
        }
    }
}


