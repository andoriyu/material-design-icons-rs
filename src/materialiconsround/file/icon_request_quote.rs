
pub struct IconRequestQuote {
  props: crate::Props,
}

impl yew::Component for IconRequestQuote {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M19.41,7.41l-4.83-4.83C14.21,2.21,13.7,2,13.17,2H6C4.9,2,4.01,2.9,4.01,4L4,20c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2 V8.83C20,8.3,19.79,7.79,19.41,7.41z M14,13c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1h-1c0,0.55-0.45,1-1,1s-1-0.45-1-1h-1 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3v-1h-3c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h1c0-0.55,0.45-1,1-1s1,0.45,1,1h1 c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1h-3v1H14z M14,8c-0.55,0-1-0.45-1-1V3.5L17.5,8H14z"/></g></svg>
            </svg>
        }
    }
}


