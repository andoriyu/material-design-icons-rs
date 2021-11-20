
pub struct IconMonitorHeart {
  props: crate::Props,
}

impl yew::Component for IconMonitorHeart {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M15.11,12.45L14,10.24l-3.11,6.21C10.73,16.79,10.38,17,10,17s-0.73-0.21-0.89-0.55L7.38,13H2v7h20v-7h-6 C15.62,13,15.27,12.79,15.11,12.45z"/><path d="M22,4H2v7h6c0.38,0,0.73,0.21,0.89,0.55L10,13.76l3.11-6.21c0.37-0.74,1.42-0.74,1.79,0L16.62,11H22V4z"/></g></g></svg>
            </svg>
        }
    }
}


