
pub struct Icon8kPlus {
  props: crate::Props,
}

impl yew::Component for Icon8kPlus {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M7.5,12.5h1V14h-1V12.5z M7.5,10h1v1.5h-1V10z M21,3H3v18h18V3z M10,14c0,0.55-0.45,1-1,1H7c-0.55,0-1-0.45-1-1v-4 c0-0.55,0.45-1,1-1h2c0.55,0,1,0.45,1,1V14z M16,15h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25,9H16l-2.25,3L16,15z M19,12.5h-1.5V14 h-1v-1.5H15v-1h1.5V10h1v1.5H19V12.5z"/></g></svg>
            </svg>
        }
    }
}


