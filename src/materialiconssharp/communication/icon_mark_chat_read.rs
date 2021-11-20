
pub struct IconMarkChatRead {
  props: crate::Props,
}

impl yew::Component for IconMarkChatRead {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" x="0"/><path d="M12.03,17.5C12.01,17.67,12,17.83,12,18v0H6l-4,4V2h20v8.68C21.09,10.25,20.08,10,19,10c-3.87,0-7,3.13-7,7 C12,17.17,12.01,17.33,12.03,17.5z M23,14.34l-1.41-1.41l-4.24,4.24l-2.12-2.12l-1.41,1.41L17.34,20L23,14.34z"/></g></svg>
            </svg>
        }
    }
}


