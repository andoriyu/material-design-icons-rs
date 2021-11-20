
pub struct IconContentPasteGo {
  props: crate::Props,
}

impl yew::Component for IconContentPasteGo {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M5,5h2v1c0,1.1,0.9,2,2,2h6c1.1,0,2-0.9,2-2V5h2v6h2V5c0-1.1-0.9-2-2-2h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5 C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h5v-2H5V5z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,3,12,3z"/><path d="M21.29,16.29l-2.58-2.58c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l0.87,0.88H13c-0.55,0-1,0.45-1,1 c0,0.55,0.45,1,1,1h5.17l-0.87,0.88c-0.39,0.39-0.39,1.02,0,1.41v0c0.39,0.39,1.02,0.39,1.41,0l2.58-2.58 C21.68,17.31,21.68,16.68,21.29,16.29z"/></g></g></svg>
            </svg>
        }
    }
}


