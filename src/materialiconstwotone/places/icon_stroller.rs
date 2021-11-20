
pub struct IconStroller {
  props: crate::Props,
}

impl yew::Component for IconStroller {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M10,5c0.29,0,0.58,0.02,0.86,0.05L9.49,6.66l-1.4-1.4C8.71,5.09,9.35,5,10,5 M15,8.66V15H9.6L15,8.66" opacity=".3"/><path d="M10,5c0.29,0,0.58,0.02,0.86,0.05L9.49,6.66l-1.4-1.4C8.71,5.09,9.35,5,10,5 M15,8.66V15H9.6L15,8.66 M18.65,3 c-1.66,0-2.54,1.27-3.18,2.03l-8.8,10.32C6.12,16,6.58,17,7.43,17H15c1.1,0,2-0.9,2-2V6.27C17.58,5.59,17.97,5,18.65,5 C19.42,5,20,5.66,20,6.48V7h2V6.48C22,4.56,20.52,3,18.65,3L18.65,3z M10,3C8.03,3,6.21,3.64,4.72,4.72l4.89,4.89l4.7-5.51 C13.03,3.4,11.56,3,10,3L10,3z M16,18c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S17.1,18,16,18L16,18z M6,18c-1.1,0-2,0.9-2,2 s0.9,2,2,2s2-0.9,2-2S7.1,18,6,18L6,18z"/></g></svg>
            </svg>
        }
    }
}

