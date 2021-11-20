
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M21,11.18V9l-5-6H1v12h2.18C3.6,16.16,4.7,17,6,17s2.4-0.84,2.82-2h8.37c0.48,1.34,1.86,2.25,3.42,1.94 c1.16-0.23,2.11-1.17,2.33-2.33C23.25,13.05,22.34,11.66,21,11.18z M18.4,9H16V6.12L18.4,9z M3,5h4v4H3V5z M6,15 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.55,15,6,15z M9,13v-2h3V9H9V5h5v8H9z M20,15c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S20.55,15,20,15z"/><polygon points="7,20 11,20 11,18 17,21 13,21 13,23"/></g></g></svg>
            </svg>
        }
    }
}


