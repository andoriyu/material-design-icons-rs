
pub struct IconFormatColorFill {
  props: crate::Props,
}

impl yew::Component for IconFormatColorFill {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M8.94,16.56C9.23,16.85,9.62,17,10,17s0.77-0.15,1.06-0.44l5.5-5.5 c0.59-0.58,0.59-1.53,0-2.12L8.32,0.7c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l1.68,1.68L3.44,8.94 c-0.59,0.59-0.59,1.54,0,2.12L8.94,16.56z M10,5.21L14.79,10H5.21L10,5.21z" enable-background="new"/><path d="M19,17c1.1,0,2-0.9,2-2c0-1.33-2-3.5-2-3.5s-2,2.17-2,3.5C17,16.1,17.9,17,19,17z" enable-background="new"/><path d="M20,20H4c-1.1,0-2,0.9-2,2s0.9,2,2,2h16c1.1,0,2-0.9,2-2S21.1,20,20,20z" enable-background="new"/></g></g></svg>
            </svg>
        }
    }
}


