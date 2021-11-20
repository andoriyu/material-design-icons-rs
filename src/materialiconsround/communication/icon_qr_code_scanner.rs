
pub struct IconQrCodeScanner {
  props: crate::Props,
}

impl yew::Component for IconQrCodeScanner {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M9.5,6.5v3h-3v-3H9.5 M11,5H5v6h6V5L11,5z M9.5,14.5v3h-3v-3H9.5 M11,13H5v6h6V13L11,13z M17.5,6.5v3h-3v-3H17.5 M19,5h-6v6 h6V5L19,5z M13,13h1.5v1.5H13V13z M14.5,14.5H16V16h-1.5V14.5z M16,13h1.5v1.5H16V13z M13,16h1.5v1.5H13V16z M14.5,17.5H16V19h-1.5 V17.5z M16,16h1.5v1.5H16V16z M17.5,14.5H19V16h-1.5V14.5z M17.5,17.5H19V19h-1.5V17.5z M21,7L21,7c-0.55,0-1-0.45-1-1V4h-2 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v3C22,6.55,21.55,7,21,7z M22,21v-3c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h3C21.55,22,22,21.55,22,21z M3,22h3c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H4v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v3C2,21.55,2.45,22,3,22z M2,3v3c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1V4h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H3C2.45,2,2,2.45,2,3z"/></svg>
            </svg>
        }
    }
}


