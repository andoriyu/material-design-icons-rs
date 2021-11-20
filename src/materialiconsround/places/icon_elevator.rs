
pub struct IconElevator {
  props: crate::Props,
}

impl yew::Component for IconElevator {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M8.5,6c0.69,0,1.25,0.56,1.25,1.25 c0,0.69-0.56,1.25-1.25,1.25S7.25,7.94,7.25,7.25C7.25,6.56,7.81,6,8.5,6z M11,13c0,0.55-0.45,1-1,1v3c0,0.55-0.45,1-1,1H8 c-0.55,0-1-0.45-1-1v-3c-0.55,0-1-0.45-1-1v-1.5c0-1.1,0.9-2,2-2h1c1.1,0,2,0.9,2,2V13z M17.52,13.76l-1.6,2.56 c-0.2,0.31-0.65,0.31-0.85,0l-1.6-2.56C13.27,13.43,13.51,13,13.9,13h3.2C17.49,13,17.73,13.43,17.52,13.76z M17.1,11h-3.2 c-0.39,0-0.63-0.43-0.42-0.77l1.6-2.56c0.2-0.31,0.65-0.31,0.85,0l1.6,2.56C17.73,10.57,17.49,11,17.1,11z"/></g></svg>
            </svg>
        }
    }
}

