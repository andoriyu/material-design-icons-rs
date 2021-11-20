
pub struct IconRecycling {
  props: crate::Props,
}

impl yew::Component for IconRecycling {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M5.77,7.15L7.2,4.78l1.03-1.71c0.39-0.65,1.33-0.65,1.72,0l1.48,2.46l-1.23,2.06L9.2,9.21L5.77,7.15z M21.72,12.97 l-1.6-2.66l-3.46,2L18.87,16H20c0.76,0,1.45-0.43,1.79-1.11C21.93,14.61,22,14.31,22,14C22,13.64,21.9,13.29,21.72,12.97z M16,21 h1.5c0.76,0,1.45-0.43,1.79-1.11L20.74,17H16v-2l-4,4l4,4V21z M10,17H5.7l-0.84,1.41c-0.3,0.5-0.32,1.12-0.06,1.65l0,0 C5.08,20.63,5.67,21,6.32,21H10V17z M6.12,14.35l1.73,1.04L6.48,9.9L1,11.27l1.7,1.02l-0.41,0.69c-0.35,0.59-0.38,1.31-0.07,1.92 l1.63,3.26L6.12,14.35z M17.02,5.14l-1.3-2.17C15.35,2.37,14.7,2,14,2h-3.53l3.12,5.2l-1.72,1.03l5.49,1.37l1.37-5.49L17.02,5.14z"/></svg>
            </svg>
        }
    }
}


