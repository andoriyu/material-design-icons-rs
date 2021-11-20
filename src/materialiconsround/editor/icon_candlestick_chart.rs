
pub struct IconCandlestickChart {
  props: crate::Props,
}

impl yew::Component for IconCandlestickChart {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M8,4L8,4C7.45,4,7,4.45,7,5v1H6C5.45,6,5,6.45,5,7v10c0,0.55,0.45,1,1,1h1v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1h1 c0.55,0,1-0.45,1-1V7c0-0.55-0.45-1-1-1H9V5C9,4.45,8.55,4,8,4z"/></g><g><path d="M18,8h-1V5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v3h-1c-0.55,0-1,0.45-1,1v5c0,0.55,0.45,1,1,1h1v4c0,0.55,0.45,1,1,1 h0c0.55,0,1-0.45,1-1v-4h1c0.55,0,1-0.45,1-1V9C19,8.45,18.55,8,18,8z"/></g></g></g></svg>
            </svg>
        }
    }
}


