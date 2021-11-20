
pub struct IconIceSkating {
  props: crate::Props,
}

impl yew::Component for IconIceSkating {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><g><path d="M21.87,17c-0.47,0-0.85,0.34-0.98,0.8C20.54,19.07,19.38,20,18,20h-2v-2h1c1.1,0,2-0.9,2-2l0-0.88 c0-2.1-1.55-3.53-3.03-3.88l-2.7-0.67C12.4,10.35,11.7,9.76,11.32,9H8.5C8.22,9,8,8.78,8,8.5S8.22,8,8.5,8h2.52L11,7H8.5 C8.22,7,8,6.78,8,6.5C8,6.22,8.22,6,8.5,6H11V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5v11c0,1.1,0.9,2,2,2h1v2H3c-0.55,0-1,0.45-1,1 s0.45,1,1,1h15c2.33,0,4.29-1.6,4.84-3.75C23.01,17.62,22.52,17,21.87,17z M14,20H8v-2h6V20z"/></g></svg>
            </svg>
        }
    }
}


