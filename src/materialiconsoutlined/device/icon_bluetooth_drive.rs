
pub struct IconBluetoothDrive {
  props: crate::Props,
}

impl yew::Component for IconBluetoothDrive {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><circle cx="6.5" cy="14.5" r="1.5"/><circle cx="15.5" cy="14.5" r="1.5"/><path d="M18,17H4v-5h11v-2H4.81l1.04-3H15V5H5.5C4.84,5,4.29,5.42,4.08,6.01L2,12v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h12 v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-8h-2V17z"/><path d="M22,3.85L19.15,1h-0.5v3.79l-2.3-2.29l-0.7,0.7L18.44,6l-2.79,2.79l0.7,0.71l2.3-2.3V11h0.5L22,8.14L19.85,6L22,3.85z M19.65,2.91l0.94,0.94l-0.94,0.94V2.91z M20.59,8.14l-0.94,0.94V7.2L20.59,8.14z"/></g></g></svg>
            </svg>
        }
    }
}

