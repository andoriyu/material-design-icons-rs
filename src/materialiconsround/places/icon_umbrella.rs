
pub struct IconUmbrella {
  props: crate::Props,
}

impl yew::Component for IconUmbrella {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M17.12,6.28L14.5,6.92L13,5.77V3.88V3.4c0-0.26,0.22-0.48,0.5-0.48c0.23,0,0.43,0.16,0.49,0.36C14.1,3.7,14.49,4,14.94,4 c0.55,0,1-0.45,1-1c0-0.1-0.02-0.2-0.05-0.3C15.59,1.72,14.63,1,13.5,1C12.12,1,11,2.07,11,3.4v0.48v1.89L9.5,6.92L6.88,6.28 C6.5,6.19,6.16,6.55,6.28,6.92l4.77,14.39C11.2,21.77,11.6,22,12,22s0.8-0.23,0.95-0.69l4.77-14.39 C17.84,6.55,17.5,6.19,17.12,6.28z M11,14.8L9.03,8.86l0.92,0.23l0.76-0.58L11,8.29V14.8z M13,14.8V8.29l0.28,0.22l0.76,0.58 l0.92-0.23L13,14.8z"/></g></svg>
            </svg>
        }
    }
}


