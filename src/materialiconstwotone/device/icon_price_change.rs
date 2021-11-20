
pub struct IconPriceChange {
  props: crate::Props,
}

impl yew::Component for IconPriceChange {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M4,18h16V6H4V18z M16,8l2,2h-4L16,8z M18,14.25l-2,2l-2-2H18z M6,14h4v-1H7c-0.55,0-1-0.45-1-1V9 c0-0.55,0.45-1,1-1h1V7h2v1h2v2H8v1h3c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1h-1v1H8v-1H6V14z" opacity=".3"/><path d="M20,4H4C2.89,4,2.01,4.89,2.01,6L2,18c0,1.11,0.89,2,2,2h16c1.11,0,2-0.89,2-2V6C22,4.89,21.11,4,20,4z M20,18H4V6h16V18z"/><path d="M8,17h2v-1h1c0.55,0,1-0.45,1-1v-3c0-0.55-0.45-1-1-1H8v-1h4V8h-2V7H8v1H7C6.45,8,6,8.45,6,9v3c0,0.55,0.45,1,1,1h3v1H6v2 h2V17z"/><polygon points="16,8 14,10 18,10"/><polygon points="18,14.25 14,14.25 16,16.25"/></g></g></svg>
            </svg>
        }
    }
}


