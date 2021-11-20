
pub struct IconTty {
  props: crate::Props,
}

impl yew::Component for IconTty {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" x="0"/><path d="M20,15v6c-3.28,0-6.35-0.89-9-2.43C8.27,16.99,6.01,14.73,4.43,12C2.89,9.35,2,6.28,2,3h6l1,5l-2.9,2.9 c1.43,2.5,3.5,4.57,6,6L15,14L20,15z M14,6h2V4h-2V6z M13,9h2V7h-2V9z M11,6h2V4h-2V6z M18,7h-2v2h2V7z M19,4h-2v2h2V4z M21,7h-2v2 h2V7z M22,4h-2v2h2V4z M14,12h2v-2h-2V12z M11,12h2v-2h-2V12z M19,10h-2v2h2V10z M22,10h-2v2h2V10z"/></g></svg>
            </svg>
        }
    }
}


