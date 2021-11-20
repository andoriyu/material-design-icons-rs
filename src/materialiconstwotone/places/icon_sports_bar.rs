
pub struct IconSportsBar {
  props: crate::Props,
}

impl yew::Component for IconSportsBar {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M15,19H8l0-6.63c1.26-0.34,2.11-1.27,2.77-1.99C11.6,9.47,12.08,9,13,9l2,0V19z M7,10.5c-1.1,0-2-0.9-2-2 c0-0.85,0.55-1.6,1.37-1.88l0.8-0.27l0.36-0.76C8,4.62,8.94,4.02,10,4.02c0.79,0,1.39,0.35,1.74,0.65l0.78,0.65 c0,0,0.64-0.32,1.47-0.32c1.1,0,2,0.9,2,2c0,0-3,0-3,0C9.67,7,9.15,10.5,7,10.5C7,10.5,7,10.5,7,10.5L7,10.5z" opacity=".3"/><path d="M15,19H8l0-6.63c1.26-0.34,2.11-1.27,2.77-1.99C11.6,9.47,12.08,9,13,9l2,0V19z M10,2.02c-1.89,0-3.51,1.11-4.27,2.71 C4.15,5.26,3,6.74,3,8.5c0,1.86,1.28,3.41,3,3.86L6,21h11v-2h2c1.1,0,2-0.9,2-2v-6c0-1.1-0.9-2-2-2h-1.56C17.79,8.41,18,7.73,18,7 c0-2.21-1.79-4-4-4c-0.34,0-0.66,0.05-0.98,0.13C12.2,2.45,11.16,2.02,10,2.02L10,2.02z M7,10.5c-1.1,0-2-0.9-2-2 c0-0.85,0.55-1.6,1.37-1.88l0.8-0.27l0.36-0.76C8,4.62,8.94,4.02,10,4.02c0.79,0,1.39,0.35,1.74,0.65l0.78,0.65 c0,0,0.64-0.32,1.47-0.32c1.1,0,2,0.9,2,2c0,0-3,0-3,0C9.67,7,9.15,10.5,7,10.5C7,10.5,7,10.5,7,10.5L7,10.5z M17,17v-6h2v6H17 L17,17z"/></svg>
            </svg>
        }
    }
}


