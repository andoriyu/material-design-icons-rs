
pub struct IconEventRepeat {
  props: crate::Props,
}

impl yew::Component for IconEventRepeat {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M21,12V6c0-1.1-0.9-2-2-2h-1V3c0-0.55-0.45-1-1-1s-1,0.45-1,1v1H8V3c0-0.55-0.45-1-1-1S6,2.45,6,3v1H5C3.9,4,3,4.9,3,6v14 c0,1.1,0.9,2,2,2h7v-2H5V10h14v2H21z M15.13,20c-0.55,0-0.91,0.56-0.68,1.06C15.23,22.79,16.97,24,19,24c2.76,0,5-2.24,5-5 s-2.24-5-5-5c-1.36,0-2.6,0.55-3.5,1.43l0-0.68c0-0.41-0.34-0.75-0.75-0.75h0C14.34,14,14,14.34,14,14.75V17c0,0.55,0.45,1,1,1 h2.25c0.41,0,0.75-0.34,0.75-0.75v0c0-0.41-0.34-0.75-0.75-0.75l-0.7,0c0.63-0.62,1.5-1,2.45-1c1.93,0,3.5,1.57,3.5,3.5 s-1.57,3.5-3.5,3.5c-1.42,0-2.64-0.85-3.19-2.06C15.69,20.17,15.42,20,15.13,20z"/></g></svg>
            </svg>
        }
    }
}


