
pub struct IconNoTransfer {
  props: crate::Props,
}

impl yew::Component for IconNoTransfer {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M5.78,2.95C7.24,2.16,9.48,2,12,2c4.42,0,8,0.5,8,4v10c0,0.35-0.08,0.67-0.19,0.98L13.83,11H18V6H8.83L5.78,2.95z M20.49,21.9c-0.39,0.39-1.02,0.39-1.41,0l-1.01-1.01C17.89,20.96,17.7,21,17.5,21c-0.83,0-1.5-0.68-1.5-1.5V19H8v0.5 C8,20.33,7.33,21,6.5,21S5,20.33,5,19.5v-1.28C4.39,17.67,4,16.88,4,16V6.83l-1.9-1.9c-0.39-0.39-0.39-1.02,0-1.41 c0.39-0.39,1.02-0.39,1.41,0l16.97,16.97C20.88,20.88,20.88,21.51,20.49,21.9z M9,15.5C9,14.67,8.33,14,7.5,14S6,14.67,6,15.5 C6,16.33,6.67,17,7.5,17S9,16.33,9,15.5z M8.17,11L6,8.83V11H8.17z"/></svg>
            </svg>
        }
    }
}


