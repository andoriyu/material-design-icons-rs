
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M9,3l0.01,10.55c-0.6-0.34-1.28-0.55-2-0.55C4.79,13,3,14.79,3,17s1.79,4,4.01,4S11,19.21,11,17V7h4V3H9z M21,12.43 L17.57,9h-0.6v4.55l-2.75-2.75l-0.85,0.85L16.73,15l-3.35,3.35l0.85,0.85l2.75-2.75V21h0.6L21,17.57L18.42,15L21,12.43z M18.17,11.3l1.13,1.13l-1.13,1.13V11.3z M19.3,17.57l-1.13,1.13v-2.26L19.3,17.57z"/></g></g></svg>
            </svg>
        }
    }
}


