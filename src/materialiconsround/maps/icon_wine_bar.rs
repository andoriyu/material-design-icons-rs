
pub struct IconWineBar {
  props: crate::Props,
}

impl yew::Component for IconWineBar {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M7,3C6.45,3,6,3.45,6,4l0,5c0,2.97,2.16,5.43,5,5.91V19H9c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h-2v-4.09c2.84-0.48,5-2.94,5-5.91l0-5c0-0.55-0.45-1-1-1H7z M16,8H8l0-3h8C16,5,16,8,16,8z"/></svg>
            </svg>
        }
    }
}


