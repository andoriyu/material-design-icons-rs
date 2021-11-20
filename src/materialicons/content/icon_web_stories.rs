
pub struct IconWebStories {
  props: crate::Props,
}

impl yew::Component for IconWebStories {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M18,4c1.1,0,2,0.9,2,2v12c0,1.1-0.9,2-2,2V4z M3,20c0,1.1,0.9,2,2,2h9c1.1,0,2-0.9,2-2V4c0-1.1-0.9-2-2-2H5C3.9,2,3,2.9,3,4 V20z M22,18c0.83,0,1.5-0.67,1.5-1.5v-9C23.5,6.67,22.83,6,22,6V18z"/></svg>
            </svg>
        }
    }
}


