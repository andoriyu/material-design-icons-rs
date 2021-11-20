
pub struct IconPrecisionManufacturing {
  props: crate::Props,
}

impl yew::Component for IconPrecisionManufacturing {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M14,10V9.82l2.01,2.01l5.23-2.44l-0.63-1.36l-4.28,2L14,7.7V6.3l2.33-2.33l4.28,2l0.63-1.36l-5.23-2.44L14,4.18V4h-2v2 H8.82C8.4,4.84,7.3,4,6,4C4.34,4,3,5.34,3,7c0,1.1,0.6,2.05,1.48,2.58L7.08,18H4v3h13v-3h-3.62L8.41,8.76 C8.58,8.53,8.72,8.28,8.82,8H12v2H14z M6,8C5.45,8,5,7.55,5,7c0-0.55,0.45-1,1-1s1,0.45,1,1C7,7.55,6.55,8,6,8z"/></g></svg>
            </svg>
        }
    }
}

