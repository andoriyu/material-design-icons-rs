
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M18,20c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2S18,18.9,18,20z M6,18c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S7.1,18,6,18z M14.3,4.1C13.03,3.4,11.56,3,10,3C8.49,3,7.07,3.38,5.83,4.03C5.24,4.34,5.15,5.15,5.61,5.61l3.99,3.99L14.3,4.1z M21.94,5.83 C21.65,4.22,20.3,3,18.65,3c-1.66,0-2.54,1.27-3.18,2.03L6.71,15.31c-0.55,0.65-0.09,1.65,0.76,1.65H15c1.1,0,2-0.9,2-2V6.27 C17.58,5.59,17.97,5,18.65,5c0.68,0,1.22,0.52,1.33,1.21l0,0C20.08,6.66,20.48,7,20.96,7c0.55,0,1-0.45,1-1 C21.96,5.94,21.95,5.89,21.94,5.83L21.94,5.83z"/></g></svg>
            </svg>
        }
    }
}


