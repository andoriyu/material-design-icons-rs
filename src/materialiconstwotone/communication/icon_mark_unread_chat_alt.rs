
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M4,17.17L5.17,16H20V7.9C19.68,7.96,19.34,8,19,8s-0.68-0.04-1-0.1V8H6V6h9.03 c-0.44-0.58-0.77-1.26-0.92-2H4V17.17z M6,9h12v2H6V9z M6,12h8v2H6V12z" enable-background="new" opacity=".3"/><g><circle cx="19" cy="3" r="3"/><path d="M20,16H5.17L4,17.17V4h10.1c-0.18-0.89-0.08-1.61,0-2H4C2.9,2,2.01,2.9,2.01,4L2,22l4-4h14c1.1,0,2-0.9,2-2V6.97 c-0.58,0.44-1.26,0.77-2,0.92V16z"/><rect height="2" width="8" x="6" y="12"/><rect height="2" width="12" x="6" y="9"/><path d="M6,8h12V7.9c-1.21-0.25-2.25-0.95-2.97-1.9H6V8z"/></g></g></g></svg>
            </svg>
        }
    }
}


