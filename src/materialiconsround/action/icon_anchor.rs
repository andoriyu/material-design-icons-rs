
pub struct IconAnchor {
  props: crate::Props,
}

impl yew::Component for IconAnchor {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M13,9V7.82C14.16,7.4,15,6.3,15,5c0-1.65-1.35-3-3-3S9,3.35,9,5c0,1.3,0.84,2.4,2,2.82V9H9c-0.55,0-1,0.45-1,1v0 c0,0.55,0.45,1,1,1h2v8.92c-2.22-0.33-4.59-1.68-5.55-3.37l1.14-1.14c0.22-0.22,0.19-0.57-0.05-0.75L3.8,12.6 C3.47,12.35,3,12.59,3,13v2c0,3.88,4.92,7,9,7s9-3.12,9-7v-2c0-0.41-0.47-0.65-0.8-0.4l-2.74,2.05c-0.24,0.18-0.27,0.54-0.05,0.75 l1.14,1.14c-0.96,1.69-3.33,3.04-5.55,3.37V11h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H13z M12,4c0.55,0,1,0.45,1,1s-0.45,1-1,1 s-1-0.45-1-1S11.45,4,12,4z"/></g></svg>
            </svg>
        }
    }
}

