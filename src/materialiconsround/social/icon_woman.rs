
pub struct IconWoman {
  props: crate::Props,
}

impl yew::Component for IconWoman {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><circle cx="12" cy="4" r="2"/><path d="M16.45,14.63l-2.52-6.32c-0.32-0.79-1.08-1.3-1.94-1.31c-0.85,0-1.62,0.51-1.94,1.31l-2.52,6.32 C7.28,15.29,7.77,16,8.47,16H10v5c0,0.55,0.45,1,1,1h1h1c0.55,0,1-0.45,1-1v-5h1.53C16.23,16,16.72,15.29,16.45,14.63z"/></g></svg>
            </svg>
        }
    }
}


