
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><g opacity=".3"><path d="M5,16V5h5v1H8.5C8.22,6,8,6.22,8,6.5C8,6.78,8.22,7,8.5,7H10l0.1,1H8.5C8.22,8,8,8.22,8,8.5C8,8.78,8.22,9,8.5,9h1.81 c0.45,1.12,1.4,2.01,2.6,2.36l2.62,0.73C16.4,12.33,17,13.1,17,14v2H5z"/></g><g><path d="M21,17c0,1.66-1.34,3-3,3h-2v-2h3v-4c0-1.79-1.19-3.34-2.91-3.82l-2.62-0.74C12.62,9.19,12,8.39,12,7.5V3H3v15h3v2H2v2h16 c2.76,0,5-2.24,5-5H21z M5,16V5h5v1H8.5C8.22,6,8,6.22,8,6.5C8,6.78,8.22,7,8.5,7H10l0.1,1H8.5C8.22,8,8,8.22,8,8.5 C8,8.78,8.22,9,8.5,9h1.81c0.45,1.12,1.4,2.01,2.6,2.36l2.62,0.73C16.4,12.33,17,13.1,17,14v2H5z M14,20H8v-2h6V20z"/></g></svg>
            </svg>
        }
    }
}


