
pub struct IconWbShade {
  props: crate::Props,
}

impl yew::Component for IconWbShade {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M14,14.13L14,14.13c0,0.23,0.09,0.46,0.26,0.63l4.98,4.98c0.17,0.17,0.39,0.26,0.62,0.26h0c0.79,0,1.18-0.95,0.62-1.51 l-4.98-4.98C14.95,12.95,14,13.35,14,14.13z M15,20h2l-3-3v2C14,19.55,14.45,20,15,20z M7.65,4.35L2.85,9.15 C2.54,9.46,2.76,10,3.21,10H4v9c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-9h0.79c0.45,0,0.67-0.54,0.35-0.85L8.35,4.35 C8.16,4.16,7.84,4.16,7.65,4.35z M9,14H7v-4h2V14z"/></g></g></svg>
            </svg>
        }
    }
}


