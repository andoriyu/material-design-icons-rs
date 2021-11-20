
pub struct IconHotelClass {
  props: crate::Props,
}

impl yew::Component for IconHotelClass {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><polygon opacity=".3" points="11,8.89 11.94,12 14.76,12 12.49,13.62 13.42,16.63 11,14.79 8.58,16.63 9.51,13.62 7.24,12 10.06,12"/><path d="M11,8.89L11.94,12h2.82l-2.27,1.62l0.93,3.01L11,14.79l-2.42,1.84l0.93-3.01L7.24,12h2.82L11,8.89z M8.58,10H1l6.17,4.41 L4.83,22L11,17.31L17.18,22l-2.35-7.59L21,10h-7.58L11,2L8.58,10z M21.36,22l-1.86-6.01L23.68,13h-3.44l-3.08,2.2l1.46,4.72 L21.36,22z M17,8l-1.82-6l-1.04,3.45L14.91,8H17z"/></svg>
            </svg>
        }
    }
}


