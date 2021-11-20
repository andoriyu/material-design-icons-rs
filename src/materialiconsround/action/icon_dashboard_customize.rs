
pub struct IconDashboardCustomize {
  props: crate::Props,
}

impl yew::Component for IconDashboardCustomize {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M4,3h6c0.55,0,1,0.45,1,1v6c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1V4C3,3.45,3.45,3,4,3z M14,3h6c0.55,0,1,0.45,1,1v6 c0,0.55-0.45,1-1,1h-6c-0.55,0-1-0.45-1-1V4C13,3.45,13.45,3,14,3z M4,13h6c0.55,0,1,0.45,1,1v6c0,0.55-0.45,1-1,1H4 c-0.55,0-1-0.45-1-1v-6C3,13.45,3.45,13,4,13z M17,13L17,13c-0.55,0-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2v-2C18,13.45,17.55,13,17,13z"/></g></svg>
            </svg>
        }
    }
}


