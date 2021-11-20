
pub struct IconMediaBluetoothOff {
  props: crate::Props,
}

impl yew::Component for IconMediaBluetoothOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M9,6.17V5c0-1.1,0.9-2,2-2h2c1.1,0,2,0.9,2,2v0c0,1.1-0.9,2-2,2h-2v1.17L9,6.17z M19.42,15l2.18,2.17 c0.22,0.22,0.22,0.58,0,0.8l0,0c-0.22,0.22-0.58,0.22-0.8,0l-5.98-5.98c-0.22-0.22-0.22-0.58,0-0.8l0,0c0.22-0.22,0.58-0.22,0.8,0 l2.35,2.35V9.61c0-0.45,0.54-0.67,0.85-0.35l2.82,2.82c0.2,0.2,0.2,0.51,0,0.71L19.42,15z M19.17,13.55l1.13-1.13l-1.13-1.13 V13.55z M20.49,20.49c0.39,0.39,0.39,1.02,0,1.41l0,0c-0.39,0.39-1.02,0.39-1.41,0l-3.28-3.28l-0.16,0.16 c-0.23,0.23-0.62,0.23-0.85,0l0,0c-0.23-0.23-0.23-0.62,0-0.85l0.16-0.16L11,13.83l0,3.02c0,2.07-1.68,4.01-3.74,4.14 C4.94,21.13,3,19.29,3,17c0-2.21,1.79-4,4.01-4c0.73,0,1.41,0.21,2,0.55v-1.72L2.1,4.92c-0.39-0.39-0.39-1.02,0-1.41l0,0 c0.39-0.39,1.02-0.39,1.41,0L20.49,20.49z"/></g></g></svg>
            </svg>
        }
    }
}

