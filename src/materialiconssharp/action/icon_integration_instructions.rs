
pub struct IconIntegrationInstructions {
  props: crate::Props,
}

impl yew::Component for IconIntegrationInstructions {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><circle cx="12" cy="3.5" fill="none" r=".75"/><circle cx="12" cy="3.5" fill="none" r=".75"/><circle cx="12" cy="3.5" fill="none" r=".75"/><path d="M21,3h-6.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H3v18h18V3z M11,14.17l-1.41,1.42L6,12l3.59-3.59L11,9.83L8.83,12 L11,14.17z M12,4.25c-0.41,0-0.75-0.34-0.75-0.75S11.59,2.75,12,2.75s0.75,0.34,0.75,0.75S12.41,4.25,12,4.25z M14.41,15.59 L13,14.17L15.17,12L13,9.83l1.41-1.42L18,12L14.41,15.59z"/></g></svg>
            </svg>
        }
    }
}

