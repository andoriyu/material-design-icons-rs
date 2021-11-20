
pub struct IconClosedCaptionOff {
  props: crate::Props,
}

impl yew::Component for IconClosedCaptionOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g enable-background="new"><path d="M19,4H5C3.89,4,3,4.9,3,6v12c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6C21,4.9,20.1,4,19,4z M11,10.5 c0,0.28-0.22,0.5-0.5,0.5H10c-0.28,0-0.5-0.22-0.5-0.5h-2v3h2c0-0.28,0.22-0.5,0.5-0.5h0.5c0.28,0,0.5,0.22,0.5,0.5V14 c0,0.55-0.45,1-1,1H7c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V10.5z M18,10.5c0,0.28-0.22,0.5-0.5,0.5H17 c-0.28,0-0.5-0.22-0.5-0.5h-2v3h2c0-0.28,0.22-0.5,0.5-0.5h0.5c0.28,0,0.5,0.22,0.5,0.5V14c0,0.55-0.45,1-1,1h-3 c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V10.5z"/></g></g></svg>
            </svg>
        }
    }
}


