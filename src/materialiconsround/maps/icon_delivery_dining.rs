
pub struct IconDeliveryDining {
  props: crate::Props,
}

impl yew::Component for IconDeliveryDining {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><g><path d="M19,7c0-1.1-0.9-2-2-2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2.65L13.52,14H10v-4c0-0.55-0.45-1-1-1H6 c-2.21,0-4,1.79-4,4v2c0,0.55,0.45,1,1,1h1c0,1.66,1.34,3,3,3s3-1.34,3-3h3.52c0.61,0,1.18-0.28,1.56-0.75l3.48-4.35 C18.85,10.54,19,10.1,19,9.65V7z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z"/><path d="M6,6h3c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H6C5.45,8,5,7.55,5,7v0C5,6.45,5.45,6,6,6z"/><path d="M19,13c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,13,19,13z M19,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 C20,16.55,19.55,17,19,17z"/></g></g></g></svg>
            </svg>
        }
    }
}


