
pub struct IconDocumentScanner {
  props: crate::Props,
}

impl yew::Component for IconDocumentScanner {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M3,6C2.45,6,2,5.55,2,5V2c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1S6.55,3,6,3H4v2C4,5.55,3.55,6,3,6z M17,2 c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V2c0-0.55-0.45-1-1-1h-3C17.45,1,17,1.45,17,2z M3,18c-0.55,0-1,0.45-1,1v3 c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H4v-2C4,18.45,3.55,18,3,18z M17,22c0,0.55,0.45,1,1,1h3 c0.55,0,1-0.45,1-1v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v2h-2C17.45,21,17,21.45,17,22z M19,18c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V6 c0-1.1,0.9-2,2-2h10c1.1,0,2,0.9,2,2V18z M9,9c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,8,9,8.45,9,9z M9,12c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,11,9,11.45,9,12z M9,15c0,0.55,0.45,1,1,1h4 c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,14,9,14.45,9,15z"/></svg>
            </svg>
        }
    }
}


