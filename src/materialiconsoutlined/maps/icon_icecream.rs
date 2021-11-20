
pub struct IconIcecream {
  props: crate::Props,
}

impl yew::Component for IconIcecream {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><path d="M18.38,6.24C17.79,3.24,15.14,1,12,1S6.21,3.24,5.62,6.24C4.08,6.81,3,8.29,3,10c0,2.21,1.79,4,4,4 c0.12,0,0.23-0.02,0.34-0.02L12.07,23l4.61-9.03C16.79,13.98,16.89,14,17,14c2.21,0,4-1.79,4-4C21,8.29,19.92,6.81,18.38,6.24z M12.05,18.63l-2.73-5.21C10.15,13.79,11.06,14,12,14c0.95,0,1.88-0.21,2.72-0.6L12.05,18.63z M17,12c-0.52,0-1.01-0.2-1.39-0.56 l-0.56-0.54l-0.66,0.42C13.68,11.76,12.86,12,12,12s-1.68-0.24-2.39-0.69L8.95,10.9l-0.56,0.54C8.01,11.79,7.52,12,7,12 c-1.1,0-2-0.89-2-2c0-0.98,0.72-1.82,1.68-1.97L7.45,7.9l0.06-0.78C7.71,4.8,9.66,3,12,3s4.29,1.8,4.48,4.12l0.06,0.78l0.77,0.12 C18.28,8.18,19,9.01,19,10C19,11.1,18.1,12,17,12z"/></g></g></svg>
            </svg>
        }
    }
}


