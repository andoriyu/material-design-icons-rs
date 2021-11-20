
pub struct IconBrunchDining {
  props: crate::Props,
}

impl yew::Component for IconBrunchDining {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><path d="M18,8h2V4h-2V8z M15,22H3c-0.55,0-1-0.45-1-1v-1h14v1C16,21.55,15.55,22,15,22z M18,15.89l-0.4-0.42 c-1.03-1.08-1.6-2.51-1.6-4V3c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v8.51c0,1.46-0.54,2.87-1.53,3.94L20,15.97V20h1 c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h-2c-0.55,0-1-0.45-1-1V15.89z M7,16v-1c0-0.55,0.45-1,1-1h2c0.55,0,1,0.45,1,1v1h4 c0.55,0,1,0.45,1,1v1H2v-1c0-0.55,0.45-1,1-1H7z"/></g></g></svg>
            </svg>
        }
    }
}


