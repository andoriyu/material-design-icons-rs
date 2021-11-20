
pub struct IconSegment {
  props: crate::Props,
}

impl yew::Component for IconSegment {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M10,18h10c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H10c-0.55,0-1,0.45-1,1v0C9,17.55,9.45,18,10,18z M3,7L3,7 c0,0.55,0.45,1,1,1h16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4C3.45,6,3,6.45,3,7z M10,13h10c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H10c-0.55,0-1,0.45-1,1v0C9,12.55,9.45,13,10,13z"/></g></svg>
            </svg>
        }
    }
}


