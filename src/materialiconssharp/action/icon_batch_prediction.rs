
pub struct IconBatchPrediction {
  props: crate::Props,
}

impl yew::Component for IconBatchPrediction {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" x="0"/><path d="M19,8H5v14h14V8z M13,20.5h-2V19h2V20.5z M13,18h-2c0-1.5-2.5-3-2.5-5c0-1.93,1.57-3.5,3.5-3.5c1.93,0,3.5,1.57,3.5,3.5 C15.5,15,13,16.5,13,18z M18,6.5H6V5h12V6.5z M17,3.5H7V2h10V3.5z"/></g></svg>
            </svg>
        }
    }
}


