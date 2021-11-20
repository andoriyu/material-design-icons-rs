
pub struct IconMediaBluetoothOn {
  props: crate::Props,
}

impl yew::Component for IconMediaBluetoothOn {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M9,5l0.01,8.55c-0.6-0.34-1.28-0.55-2-0.55C4.79,13,3,14.79,3,17s1.79,4,4.01,4S11,19.21,11,17V7h2c1.1,0,2-0.9,2-2v0 c0-1.1-0.9-2-2-2h-2C9.9,3,9,3.9,9,5z M20.29,11.72l-2.47-2.47c-0.32-0.31-0.85-0.09-0.85,0.35v3.94l-2.33-2.33 c-0.23-0.23-0.61-0.23-0.85,0l0,0c-0.23,0.23-0.23,0.62,0,0.85L16.73,15l-2.93,2.93c-0.23,0.23-0.23,0.61,0,0.85l0,0 c0.23,0.23,0.61,0.23,0.85,0l2.33-2.33v3.94c0,0.45,0.54,0.67,0.85,0.35l2.46-2.46c0.39-0.39,0.39-1.02,0-1.41L18.42,15l1.87-1.86 C20.68,12.75,20.68,12.11,20.29,11.72z M18.17,11.3l1.13,1.13l-1.13,1.13V11.3z M19.3,17.57l-1.13,1.13v-2.26L19.3,17.57z"/></g></g></svg>
            </svg>
        }
    }
}


