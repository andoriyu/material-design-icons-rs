
pub struct IconGrading {
  props: crate::Props,
}

impl yew::Component for IconGrading {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M5,7h14c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H5C4.45,9,4,8.55,4,8v0C4,7.45,4.45,7,5,7z"/><path d="M5,13h14c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H5c-0.55,0-1,0.45-1,1v0C4,12.55,4.45,13,5,13z"/><path d="M5,17h5c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H5c-0.55,0-1,0.45-1,1v0C4,16.55,4.45,17,5,17z"/><path d="M5,21h5c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H5c-0.55,0-1,0.45-1,1v0C4,20.55,4.45,21,5,21z"/><path d="M15.41,18.17l-0.71-0.71c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l1.42,1.42 c0.39,0.39,1.02,0.39,1.41,0l3.17-3.17c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0L15.41,18.17z"/><path d="M4,4L4,4c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H5C4.45,3,4,3.45,4,4z"/></g></svg>
            </svg>
        }
    }
}


