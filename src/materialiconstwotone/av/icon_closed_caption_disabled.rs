
pub struct IconClosedCaptionDisabled {
  props: crate::Props,
}

impl yew::Component for IconClosedCaptionDisabled {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M8.83,6H19v10.17l-1.4-1.4C17.84,14.59,18,14.32,18,14v-1h-1.5v0.5h-0.17 l-1.83-1.83V10.5h2V11H18v-1c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v0.17L8.83,6z M7.5,13.5h2V13h0.67l-2.5-2.5H7.5V13.5z M11,14 c0,0.55-0.45,1-1,1H7c-0.55,0-1-0.45-1-1v-4c0-0.32,0.16-0.59,0.4-0.78L5,7.83V18h10.17L11,13.83V14z" enable-background="new" opacity=".3"/><path d="M6.83,4H19c1.1,0,2,0.9,2,2v12c0,0.05-0.01,0.1-0.02,0.16L19,16.17V6H8.83L6.83,4z M19.78,22.61L17.17,20H5 c-1.11,0-2-0.9-2-2V6c0-0.05,0.02-0.1,0.02-0.15L1.39,4.22l1.41-1.41L18,18l1.82,1.82l1.37,1.37L19.78,22.61z M7.5,13.5h2V13h0.67 l-2.5-2.5H7.5V13.5z M15.17,18L11,13.83V14c0,0.55-0.45,1-1,1H7c-0.55,0-1-0.45-1-1v-4c0-0.32,0.16-0.59,0.4-0.78L5,7.83V18H15.17z M18,14v-1h-1.5v0.5h-0.17l1.28,1.28C17.84,14.59,18,14.32,18,14z M14.5,11.67V10.5h2V11H18v-1c0-0.55-0.45-1-1-1h-3 c-0.55,0-1,0.45-1,1v0.17L14.5,11.67z"/></svg>
            </svg>
        }
    }
}

