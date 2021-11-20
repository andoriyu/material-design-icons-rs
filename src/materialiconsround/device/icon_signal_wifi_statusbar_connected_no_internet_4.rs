
pub struct IconSignalWifiStatusbarConnectedNoInternet4 {
  props: crate::Props,
}

impl yew::Component for IconSignalWifiStatusbarConnectedNoInternet4 {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M22.92,8H17v7.99l-4.29,4.3c-0.39,0.39-1.02,0.39-1.42,0L0.73,9.71C0.32,9.3,0.35,8.63,0.79,8.24C3.78,5.6,7.7,4,12,4 C16.16,4,19.97,5.51,22.92,8z M20,18c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S20.55,18,20,18z M20,10c-0.55,0-1,0.45-1,1v4 c0,0.55,0.45,1,1,1s1-0.45,1-1v-4C21,10.45,20.55,10,20,10z"/></svg>
            </svg>
        }
    }
}


