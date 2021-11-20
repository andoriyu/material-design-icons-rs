
pub struct IconScreenSearchDesktop {
  props: crate::Props,
}

impl yew::Component for IconScreenSearchDesktop {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M22,19H2c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h20c0.55,0,1-0.45,1-1C23,19.45,22.55,19,22,19z"/><path d="M4,18h16c1.1,0,1.99-0.9,1.99-2L22,5c0-1.1-0.9-2-2-2H4C2.9,3,2,3.9,2,5v11C2,17.1,2.9,18,4,18z M8.59,8.05 C9.87,6.18,12.45,6,13.97,7.53c1.18,1.18,1.34,3,0.47,4.36L16,13.44c0.29,0.29,0.29,0.77,0,1.06c-0.29,0.29-0.77,0.29-1.06,0 l-1.55-1.55c-1.57,1-3.76,0.64-4.87-1.11C7.79,10.7,7.83,9.17,8.59,8.05z"/><circle cx="11.5" cy="10" r="2"/></g></g></svg>
            </svg>
        }
    }
}


