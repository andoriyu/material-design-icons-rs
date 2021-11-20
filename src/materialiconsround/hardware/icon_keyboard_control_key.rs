
pub struct IconKeyboardControlKey {
  props: crate::Props,
}

impl yew::Component for IconKeyboardControlKey {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M5.71,12.71L5.71,12.71c0.39,0.39,1.02,0.39,1.41,0L12,7.83l4.88,4.88c0.39,0.39,1.02,0.39,1.41,0v0 c0.39-0.39,0.39-1.02,0-1.41l-5.59-5.59c-0.39-0.39-1.02-0.39-1.41,0l-5.59,5.59C5.32,11.68,5.32,12.32,5.71,12.71z"/></g></svg>
            </svg>
        }
    }
}


