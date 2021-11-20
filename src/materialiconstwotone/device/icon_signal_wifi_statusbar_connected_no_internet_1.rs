
pub struct IconSignalWifiStatusbarConnectedNoInternet1 {
  props: crate::Props,
}

impl yew::Component for IconSignalWifiStatusbarConnectedNoInternet1 {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M17,14.76V8h5.92C19.97,5.51,16.16,4,12,4C7.31,4,3.07,5.9,0,8.98l6.35,6.36C7.8,13.89,9.79,13,12,13 C13.89,13,15.63,13.66,17,14.76z" fill-opacity=".3"/><g><path d="M6.35,15.34L12,21l5-5.01v-1.23c-1.37-1.1-3.11-1.76-5-1.76C9.79,13,7.8,13.89,6.35,15.34z"/><rect height="2" width="2" x="19" y="18"/><rect height="6" width="2" x="19" y="10"/></g></g></g></svg>
            </svg>
        }
    }
}


