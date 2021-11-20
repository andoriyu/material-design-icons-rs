
pub struct IconDynamicForm {
  props: crate::Props,
}

impl yew::Component for IconDynamicForm {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M13,11H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h9V11z M4,9h7V6H4V9z M15,20H4c-1.1,0-2-0.9-2-2v-3c0-1.1,0.9-2,2-2h11V20z M4,18h9v-3H4V18z M22,9h-2l2-5h-7v7h2v9L22,9z M4.75,17.25h1.5v-1.5h-1.5V17.25z M4.75,8.25h1.5v-1.5h-1.5V8.25z"/></g></svg>
            </svg>
        }
    }
}


