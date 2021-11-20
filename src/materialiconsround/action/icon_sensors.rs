
pub struct IconSensors {
  props: crate::Props,
}

impl yew::Component for IconSensors {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M8.54,8.54c0.35,0.35,0.37,0.88,0.1,1.29C8.24,10.45,8,11.2,8,12c0,0.8,0.24,1.55,0.64,2.17c0.27,0.41,0.24,0.95-0.11,1.29 c-0.43,0.43-1.17,0.4-1.51-0.11C6.38,14.4,6,13.24,6,12c0-1.21,0.36-2.33,0.97-3.28C7.33,8.18,8.08,8.08,8.54,8.54z M15.46,15.46 c0.43,0.43,1.17,0.4,1.51-0.11C17.62,14.4,18,13.24,18,12c0-1.24-0.38-2.4-1.03-3.36c-0.34-0.5-1.08-0.54-1.51-0.11 c-0.35,0.35-0.37,0.88-0.11,1.29C15.76,10.45,16,11.2,16,12c0,0.8-0.24,1.55-0.64,2.17C15.09,14.58,15.12,15.12,15.46,15.46z M12,10 c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S13.1,10,12,10z M18.32,18.32c0.42,0.42,1.12,0.39,1.5-0.08C21.18,16.53,22,14.36,22,12 s-0.82-4.53-2.18-6.24c-0.37-0.47-1.07-0.5-1.5-0.08c-0.36,0.36-0.4,0.92-0.08,1.32c1.1,1.37,1.76,3.11,1.76,5s-0.66,3.63-1.76,5 C17.92,17.39,17.96,17.96,18.32,18.32z M5.68,5.68c-0.42-0.42-1.12-0.39-1.5,0.08C2.82,7.47,2,9.64,2,12s0.82,4.53,2.18,6.24 c0.37,0.47,1.07,0.5,1.5,0.08c0.36-0.36,0.4-0.92,0.08-1.32C4.66,15.63,4,13.89,4,12s0.66-3.63,1.76-5 C6.08,6.61,6.04,6.04,5.68,5.68z"/></svg>
            </svg>
        }
    }
}


