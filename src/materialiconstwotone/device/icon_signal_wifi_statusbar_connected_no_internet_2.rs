
pub struct IconSignalWifiStatusbarConnectedNoInternet2 {
  props: crate::Props,
}

impl yew::Component for IconSignalWifiStatusbarConnectedNoInternet2 {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M17,11.21V8h5.92C19.97,5.51,16.16,4,12,4C7.31,4,3.07,5.9,0,8.98l4.23,4.24C6.22,11.23,8.97,10,12,10 C13.8,10,15.5,10.44,17,11.21z" fill-opacity=".3"/><g><path d="M4.23,13.22L12,21l5-5.01v-4.78C15.5,10.44,13.8,10,12,10C8.97,10,6.22,11.23,4.23,13.22z"/><rect height="2" width="2" x="19" y="18"/><rect height="6" width="2" x="19" y="10"/></g></g></g></svg>
            </svg>
        }
    }
}


