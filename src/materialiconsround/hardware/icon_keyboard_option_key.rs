
pub struct IconKeyboardOptionKey {
  props: crate::Props,
}

impl yew::Component for IconKeyboardOptionKey {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M15,6L15,6c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-4C15.45,5,15,5.45,15,6z"/><path d="M9.58,6C9.22,5.38,8.56,5,7.85,5H4C3.45,5,3,5.45,3,6v0c0,0.55,0.45,1,1,1h3.85l6.35,11c0.36,0.62,1.02,1,1.73,1H20 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-4.07L9.58,6z"/></g></g></svg>
            </svg>
        }
    }
}


