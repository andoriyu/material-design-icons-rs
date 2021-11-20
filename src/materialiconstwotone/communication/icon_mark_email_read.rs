
pub struct IconMarkEmailRead {
  props: crate::Props,
}

impl yew::Component for IconMarkEmailRead {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M20,8v4.08C19.67,12.03,19.34,12,19,12c-3.53,0-6.43,2.61-6.92,6H4V8l8,5L20,8z M20,6H4l8,5L20,6z" opacity=".3"/><path d="M12.08,18H4V8l8,5l8-5v4.08c0.71,0.1,1.38,0.31,2,0.6V6c0-1.1-0.9-2-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2 h8.08C12.03,19.67,12,19.34,12,19S12.03,18.33,12.08,18z M20,6l-8,5L4,6H20z M17.34,22l-3.54-3.54l1.41-1.41l2.12,2.12l4.24-4.24 L23,16.34L17.34,22z"/></g></svg>
            </svg>
        }
    }
}

