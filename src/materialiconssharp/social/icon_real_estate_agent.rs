
pub struct IconRealEstateAgent {
  props: crate::Props,
}

impl yew::Component for IconRealEstateAgent {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M1,22h4V11H1V22z M14,1.5l-7,5V9h2l10,4v2h2V6.5L14,1.5z M13.5,10h-1V9h1V10z M13.5,8h-1V7h1V8z M15.5,10h-1V9h1V10z M15.5,8h-1V7h1V8z M22,19l-8,3l-7-1.98V11h1.97L17,14l0,2h-4l-1.76-0.68l-0.33,0.94L13,17h9V19z"/></svg>
            </svg>
        }
    }
}


