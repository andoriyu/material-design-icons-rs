
pub struct IconTurnSlightRight {
  props: crate::Props,
}

impl yew::Component for IconTurnSlightRight {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M12.34,5L12.34,5c0-0.55,0.45-1,1-1H17c0.55,0,1,0.45,1,1v3.66c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V7.41l-5,5V19 c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-6.58c0-0.53,0.21-1.04,0.59-1.41l5-5h-1.24C12.79,6,12.34,5.55,12.34,5z"/></g></svg>
            </svg>
        }
    }
}


