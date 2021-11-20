
pub struct IconWifiPassword {
  props: crate::Props,
}

impl yew::Component for IconWifiPassword {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M24,8.98l-2.12,2.13C19.35,8.57,15.85,7,12,7s-7.35,1.57-9.88,4.11L0,8.98C3.07,5.9,7.31,4,12,4S20.93,5.9,24,8.98z M24,20 v3c0,0.55-0.45,1-1,1h-4c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1v-1c0-1.1,0.9-2,2-2s2,0.9,2,2v1C23.55,19,24,19.45,24,20z M22,18 c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h2V18z M4.24,13.22l2.12,2.12C7.8,13.9,9.8,13,12,13c2.2,0,4.2,0.9,5.64,2.35l2.12-2.12 C17.78,11.23,15.03,10,12,10C8.97,10,6.22,11.23,4.24,13.22z M12,16c-1.38,0-2.63,0.56-3.53,1.46L12,21l3.53-3.54 C14.63,16.56,13.38,16,12,16z"/></g></svg>
            </svg>
        }
    }
}


