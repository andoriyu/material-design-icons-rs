
pub struct IconAutoFixNormal {
  props: crate::Props,
}

impl yew::Component for IconAutoFixNormal {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" x="0"/></g><g><g><path d="M20.45,6l0.49-1.06L22,4.45c0.39-0.18,0.39-0.73,0-0.91l-1.06-0.49L20.45,2c-0.18-0.39-0.73-0.39-0.91,0l-0.49,1.06 L18,3.55c-0.39,0.18-0.39,0.73,0,0.91l1.06,0.49L19.55,6C19.72,6.39,20.28,6.39,20.45,6z"/><path d="M17.71,9.12l-2.83-2.83c-0.39-0.39-1.02-0.39-1.41,0L2.29,17.46c-0.39,0.39-0.39,1.02,0,1.41l2.83,2.83 c0.39,0.39,1.02,0.39,1.41,0l11.17-11.17C18.1,10.15,18.1,9.51,17.71,9.12z M14.21,11.21l-1.41-1.41l1.38-1.38l1.41,1.41 L14.21,11.21z"/></g></g></svg>
            </svg>
        }
    }
}

