
pub struct IconSettingsEthernet {
  props: crate::Props,
}

impl yew::Component for IconSettingsEthernet {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M7.71,6.71L7.71,6.71c-0.39-0.39-1.02-0.39-1.41,0l-4.59,4.59c-0.39,0.39-0.39,1.02,0,1.41l4.59,4.59 c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L3.83,12l3.88-3.88C8.09,7.73,8.09,7.09,7.71,6.71z M16.29,6.71 L16.29,6.71c-0.39,0.39-0.39,1.02,0,1.41L20.17,12l-3.88,3.88c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0 l4.59-4.59c0.39-0.39,0.39-1.02,0-1.41l-4.59-4.59C17.32,6.32,16.68,6.32,16.29,6.71z M8,13L8,13c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C7,12.55,7.45,13,8,13z M12,13L12,13c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v0C11,12.55,11.45,13,12,13z M16,11L16,11c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v0 C17,11.45,16.55,11,16,11z"/></g></svg>
            </svg>
        }
    }
}


