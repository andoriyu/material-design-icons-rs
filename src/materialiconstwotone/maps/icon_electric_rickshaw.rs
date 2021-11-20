
pub struct IconElectricRickshaw {
  props: crate::Props,
}

impl yew::Component for IconElectricRickshaw {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M3,13h0.17C3.58,11.84,4.7,11,6,11c0.35,0,0.69,0.06,1,0.17V10H3V13z" enable-background="new" opacity=".3"/><path d="M19,11h-3v2h1.17c0.3-0.85,0.98-1.53,1.83-1.83V11z" enable-background="new" opacity=".3"/><path d="M21,11.18V9.72c0-0.47-0.16-0.92-0.46-1.28L16.6,3.72C16.22,3.26,15.66,3,15.06,3H3C1.9,3,1,3.9,1,5v8c0,1.1,0.9,2,2,2 h0.18C3.6,16.16,4.7,17,6,17s2.4-0.84,2.82-2h8.37c0.41,1.16,1.51,2,2.82,2c1.66,0,3-1.34,3-3C23,12.7,22.16,11.6,21,11.18z M6,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.55,15,6,15z M7,11.17C6.69,11.06,6.35,11,6,11c-1.3,0-2.42,0.84-2.83,2H3v-3 h4V11.17z M7,8H3V5h4V8z M14,13H9v-3h3V8H9V5h5V13z M16,6.12L18.4,9H16V6.12z M17.17,13H16v-2h3v0.17 C18.15,11.47,17.47,12.15,17.17,13z M20,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S20.55,15,20,15z"/></g><polygon points="7,20 11,20 11,18 17,21 13,21 13,23"/></g></g></svg>
            </svg>
        }
    }
}


