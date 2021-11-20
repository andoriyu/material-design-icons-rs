
pub struct IconNearbyError {
  props: crate::Props,
}

impl yew::Component for IconNearbyError {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M11.29,8.28l-3.01,3.01c-0.39,0.39-0.39,1.02,0,1.41l3.01,3.01c0.39,0.39,1.02,0.39,1.41,0l3.01-3.01 c0.39-0.39,0.39-1.02,0-1.41l-3.01-3.01C12.32,7.89,11.68,7.89,11.29,8.28z"/><path d="M10.59,2.59l-8.01,8c-0.78,0.78-0.78,2.05,0,2.83l8.01,8c0.78,0.78,2.05,0.78,2.83,0L18,16.82V13.2l-6,6L4.79,12L12,4.79 l6,6V7.17l-4.58-4.58C12.64,1.8,11.37,1.8,10.59,2.59z"/><path d="M20,11v6c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-6c0-0.55-0.45-1-1-1h0C20.45,10,20,10.45,20,11z"/><circle cx="21" cy="21" r="1"/></g></g></svg>
            </svg>
        }
    }
}


