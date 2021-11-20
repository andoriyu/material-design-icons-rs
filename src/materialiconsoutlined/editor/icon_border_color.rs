
pub struct IconBorderColor {
  props: crate::Props,
}

impl yew::Component for IconBorderColor {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M16.81,8.94l-3.75-3.75L4,14.25V18h3.75L16.81,8.94z M6,16v-0.92l7.06-7.06l0.92,0.92L6.92,16H6z"/><path d="M19.71,6.04c0.39-0.39,0.39-1.02,0-1.41l-2.34-2.34C17.17,2.09,16.92,2,16.66,2c-0.25,0-0.51,0.1-0.7,0.29l-1.83,1.83 l3.75,3.75L19.71,6.04z"/><rect height="4" width="20" x="2" y="20"/></g></g></svg>
            </svg>
        }
    }
}


