
pub struct IconSettingsInputComposite {
  props: crate::Props,
}

impl yew::Component for IconSettingsInputComposite {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M5,2c0-0.55-0.45-1-1-1S3,1.45,3,2v4H2C1.45,6,1,6.45,1,7v5h6V7c0-0.55-0.45-1-1-1H5V2z M9,16 c0,1.3,0.84,2.4,2,2.82V22c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-3.18c1.16-0.41,2-1.51,2-2.82v-2H9V16z M1,16 c0,1.3,0.84,2.4,2,2.82V22c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-3.18C6.16,18.4,7,17.3,7,16v-2H1V16z M21,6V2c0-0.55-0.45-1-1-1 s-1,0.45-1,1v4h-1c-0.55,0-1,0.45-1,1v5h6V7c0-0.55-0.45-1-1-1H21z M13,2c0-0.55-0.45-1-1-1s-1,0.45-1,1v4h-1C9.45,6,9,6.45,9,7v5 h6V7c0-0.55-0.45-1-1-1h-1V2z M17,16c0,1.3,0.84,2.4,2,2.82V22c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-3.18 c1.16-0.41,2-1.51,2-2.82v-2h-6V16z" enable-background="new"/></g></svg>
            </svg>
        }
    }
}


