
pub struct IconHexagon {
  props: crate::Props,
}

impl yew::Component for IconHexagon {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M16.05,3H7.95C7.24,3,6.58,3.38,6.22,4l-4.04,7c-0.36,0.62-0.36,1.38,0,2l4.04,7c0.36,0.62,1.02,1,1.73,1h8.09 c0.71,0,1.37-0.38,1.73-1l4.04-7c0.36-0.62,0.36-1.38,0-2l-4.04-7C17.42,3.38,16.76,3,16.05,3z"/></g></svg>
            </svg>
        }
    }
}


