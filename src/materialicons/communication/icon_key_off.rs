
pub struct IconKeyOff {
  props: crate::Props,
}

impl yew::Component for IconKeyOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M16.91,14.09L17,14l2,2l4-4.04L21,10h-8.17L16.91,14.09z M3.98,6.81C2.2,7.85,1,9.79,1,12c0,3.31,2.69,6,6,6 c2.21,0,4.15-1.2,5.18-2.99l7.59,7.59l1.41-1.41L2.81,2.81L1.39,4.22L3.98,6.81z M9.91,12.74C9.58,14.03,8.4,15,7,15 c-1.65,0-3-1.35-3-3c0-1.4,0.97-2.58,2.26-2.91L9.91,12.74z"/></g></svg>
            </svg>
        }
    }
}


