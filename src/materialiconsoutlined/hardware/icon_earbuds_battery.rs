
pub struct IconEarbudsBattery {
  props: crate::Props,
}

impl yew::Component for IconEarbudsBattery {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M21,7h-1V6h-2v1h-1c-0.55,0-1,0.45-1,1v9c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1V8C22,7.45,21.55,7,21,7z M20,16h-2V9h2 V16z M14,9.38C14,7.51,12.49,6,10.62,6S7.25,7.51,7.25,9.38v5.25c0,1.04-0.84,1.88-1.88,1.88S3.5,15.66,3.5,14.62v-4.7 C3.66,9.97,3.83,10,4,10c1.1,0,2-0.9,2-2S5.1,6,4,6S2,6.9,2,8c0,0.04,0,6.62,0,6.62C2,16.49,3.51,18,5.38,18s3.38-1.51,3.38-3.38 V9.38c0-1.04,0.84-1.88,1.88-1.88s1.88,0.84,1.88,1.88v4.7C12.34,14.03,12.17,14,12,14c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2 C14,15.96,14,9.38,14,9.38z"/></g></g></svg>
            </svg>
        }
    }
}


