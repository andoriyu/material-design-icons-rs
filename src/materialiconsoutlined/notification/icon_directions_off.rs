
pub struct IconDirectionsOff {
  props: crate::Props,
}

impl yew::Component for IconDirectionsOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M21.71,11.29l-9-9c-0.39-0.39-1.02-0.39-1.41,0L8.21,5.38l1.41,1.41L12,4.42L19.58,12l-2.38,2.38l1.41,1.41l3.09-3.09 C22.1,12.33,22.1,11.7,21.71,11.29z"/><polygon points="13,7.5 13,10.17 15.17,12.34 16.5,11"/><path d="M1.39,4.22l3.99,3.99l-3.09,3.09c-0.39,0.39-0.39,1.02,0,1.41l9,9c0.39,0.39,1.02,0.39,1.41,0l3.09-3.09l3.99,3.99 l1.41-1.41L2.81,2.81L1.39,4.22z M8.03,10.85C8.02,10.9,7.99,10.95,7.99,11v4h2v-2.18l4.38,4.38L12,19.58L4.42,12l2.38-2.38 L8.03,10.85z"/></g></g></svg>
            </svg>
        }
    }
}


