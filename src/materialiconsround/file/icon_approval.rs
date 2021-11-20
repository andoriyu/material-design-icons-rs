
pub struct IconApproval {
  props: crate::Props,
}

impl yew::Component for IconApproval {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M4,16v4c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-4c0-1.1-0.9-2-2-2H6C4.9,14,4,14.9,4,16z M17,18H7c-0.55,0-1-0.45-1-1v0 c0-0.55,0.45-1,1-1h10c0.55,0,1,0.45,1,1v0C18,17.55,17.55,18,17,18z M12,2C9.54,2,7.48,3.79,7.07,6.13 C6.99,6.65,7.13,7.18,7.43,7.6l3.76,5.26c0.4,0.56,1.23,0.56,1.63,0l3.76-5.26c0.3-0.42,0.44-0.95,0.35-1.47 C16.52,3.79,14.46,2,12,2z"/></g></g></svg>
            </svg>
        }
    }
}

