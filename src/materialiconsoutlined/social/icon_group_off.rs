
pub struct IconGroupOff {
  props: crate::Props,
}

impl yew::Component for IconGroupOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M15,8c0-1.42-0.5-2.73-1.33-3.76C14.09,4.1,14.53,4,15,4c2.21,0,4,1.79,4,4s-1.79,4-4,4c-0.06,0-0.12,0-0.18,0l-0.77-0.77 C14.65,10.29,15,9.18,15,8z M22.83,20H23v-3c0-2.18-3.58-3.47-6.34-3.87c1.1,0.75,1.95,1.71,2.23,2.94L22.83,20z M9,6 C8.94,6,8.89,6,8.84,6.01l-1.6-1.6C7.77,4.15,8.37,4,9,4c2.21,0,4,1.79,4,4c0,0.63-0.15,1.23-0.41,1.76l-1.6-1.6 C11,8.11,11,8.06,11,8C11,6.9,10.1,6,9,6z M9.17,12C9.11,12,9.06,12,9,12c-2.21,0-4-1.79-4-4c0-0.06,0-0.11,0-0.17L0.69,3.51 L2.1,2.1l19.8,19.8l-1.41,1.41L17,19.83V20H1v-3c0-2.66,5.33-4,8-4c0.37,0,0.8,0.03,1.25,0.08L9.17,12z M9,15 c-2.7,0-5.8,1.29-6,2.01V18h12v-0.17l-2.11-2.11C11.76,15.31,10.33,15,9,15z"/></svg>
            </svg>
        }
    }
}


