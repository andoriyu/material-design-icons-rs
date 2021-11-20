
pub struct Icon123 {
  props: crate::Props,
}

impl yew::Component for Icon123 {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M4.75,10.5C4.34,10.5,4,10.16,4,9.75S4.34,9,4.75,9H6c0.55,0,1,0.45,1,1v4.25C7,14.66,6.66,15,6.25,15S5.5,14.66,5.5,14.25 V10.5H4.75z M9.75,9C9.34,9,9,9.34,9,9.75s0.34,0.75,0.75,0.75H12v1h-2c-0.55,0-1,0.45-1,1V14c0,0.55,0.45,1,1,1h2.75 c0.41,0,0.75-0.34,0.75-0.75c0-0.41-0.34-0.75-0.75-0.75H10.5v-1h2c0.55,0,1-0.45,1-1V10c0-0.55-0.45-1-1-1H9.75z M18.5,15 c0.55,0,1-0.45,1-1v-4c0-0.55-0.45-1-1-1h-2.75C15.34,9,15,9.34,15,9.75s0.34,0.75,0.75,0.75H18v1h-1.5c-0.28,0-0.5,0.22-0.5,0.5 s0.22,0.5,0.5,0.5H18v1h-2.25c-0.41,0-0.75,0.34-0.75,0.75S15.34,15,15.75,15H18.5z"/></g></svg>
            </svg>
        }
    }
}


