
pub struct IconBalcony {
  props: crate::Props,
}

impl yew::Component for IconBalcony {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M7,16H5v4h2V16z M11,16H9v4h2V16z M6,10v4h5V4.08C8.16,4.56,6,7.03,6,10z M10,12H8v-2h2V12z M13,4.08V14h5v-4 C18,7.03,15.84,4.56,13,4.08z M16,12h-2v-2h2V12z M15,16h-2v4h2V16z M19,16h-2v4h2V16z" opacity=".3"/><path d="M10,10v2H8v-2H10z M16,12v-2h-2v2H16z M21,14v8H3v-8h1v-4c0-4.42,3.58-8,8-8s8,3.58,8,8v4H21z M7,16H5v4h2V16z M11,16H9v4h2 V16z M11,4.08C8.16,4.56,6,7.03,6,10v4h5V4.08z M13,14h5v-4c0-2.97-2.16-5.44-5-5.92V14z M15,16h-2v4h2V16z M19,16h-2v4h2V16z"/></svg>
            </svg>
        }
    }
}


