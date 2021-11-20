
pub struct IconForward5 {
  props: crate::Props,
}

impl yew::Component for IconForward5 {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M18,13c0,3.31-2.69,6-6,6s-6-2.69-6-6s2.69-6,6-6v4l5-5l-5-5v4c-4.42,0-8,3.58-8,8c0,4.42,3.58,8,8,8c4.42,0,8-3.58,8-8 H18z"/><path d="M12.03,15.38c-0.44,0-0.58-0.31-0.6-0.56h-0.84c0.03,0.85,0.79,1.25,1.44,1.25c0.93,0,1.44-0.63,1.44-1.43 c0-1.33-0.97-1.44-1.3-1.44c-0.2,0-0.43,0.05-0.64,0.16l0.11-0.92h1.7v-0.71h-2.39l-0.25,2.17l0.67,0.17 c0.13-0.13,0.28-0.23,0.57-0.23c0.4,0,0.69,0.23,0.69,0.75C12.62,14.64,12.65,15.38,12.03,15.38z"/></g></g></svg>
            </svg>
        }
    }
}


