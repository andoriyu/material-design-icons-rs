
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><polygon points="13,7.5 16.5,11 15.17,12.34 18.62,15.79 22.41,12 12.01,1.58 8.21,5.38 13,10.17"/><path d="M1.39,4.22l3.99,3.99l-3.79,3.79L12.01,22.4l3.79-3.79l3.99,3.99l1.41-1.41L2.81,2.81L1.39,4.22z M9.99,12.82V15h-2v-4.18 L9.99,12.82z"/></g></g></svg>
            </svg>
        }
    }
}


