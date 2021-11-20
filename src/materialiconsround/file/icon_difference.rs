
pub struct IconDifference {
  props: crate::Props,
}

impl yew::Component for IconDifference {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M3,7c0.55,0,1,0.45,1,1v13h13c0.55,0,1,0.45,1,1s-0.45,1-1,1H4c-1.1,0-2-0.9-2-2V8C2,7.45,2.45,7,3,7z M15.59,1.59 C15.21,1.21,14.7,1,14.17,1H8C6.9,1,6.01,1.9,6.01,3L6,17c0,1.1,0.89,2,1.99,2H19c1.1,0,2-0.9,2-2V7.83c0-0.53-0.21-1.04-0.59-1.41 L15.59,1.59z M15.5,15h-4c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0C16.5,14.55,16.05,15,15.5,15z M15.5,9h-1 v1c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V9h-1c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h1V6c0-0.55,0.45-1,1-1h0 c0.55,0,1,0.45,1,1v1h1c0.55,0,1,0.45,1,1v0C16.5,8.55,16.05,9,15.5,9z"/></g></svg>
            </svg>
        }
    }
}


