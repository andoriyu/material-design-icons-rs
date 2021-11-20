
pub struct IconMarkUnreadChatAlt {
  props: crate::Props,
}

impl yew::Component for IconMarkUnreadChatAlt {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><circle cx="19" cy="3" r="3"/><path d="M7,8C6.45,8,6,7.55,6,7c0-0.55,0.45-1,1-1h8.03c-1.21-1.6-1.08-3.21-0.92-4H4.01c-1.1,0-2,0.89-2,2L2,19.58 c0,0.89,1.08,1.34,1.71,0.71L6,18h14c1.1,0,2-0.9,2-2V6.97C21.16,7.61,20.13,8,19,8H7z M13,14H7c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1C14,13.55,13.55,14,13,14z M17,11H7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h10 c0.55,0,1,0.45,1,1C18,10.55,17.55,11,17,11z"/></g></g></svg>
            </svg>
        }
    }
}


