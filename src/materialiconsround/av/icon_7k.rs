
pub struct Icon7k {
  props: crate::Props,
}

impl yew::Component for Icon7k {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M8.95,15H8.76 c-0.5,0-0.86-0.49-0.72-0.97l1.08-3.53H7.25c-0.41,0-0.75-0.34-0.75-0.75v0C6.5,9.34,6.84,9,7.25,9H10c0.67,0,1.15,0.65,0.96,1.29 l-1.3,4.18C9.57,14.79,9.28,15,8.95,15z M16.59,15L16.59,15c-0.22,0-0.42-0.1-0.55-0.27l-1.54-1.98v1.55c0,0.39-0.31,0.7-0.7,0.7 H13.7c-0.39,0-0.7-0.31-0.7-0.7V9.7C13,9.31,13.31,9,13.7,9h0.09c0.39,0,0.7,0.31,0.7,0.7v1.55l1.54-1.98 C16.17,9.1,16.38,9,16.59,9l0,0c0.58,0,0.91,0.66,0.56,1.12L15.75,12l1.41,1.88C17.5,14.34,17.17,15,16.59,15z"/></g></svg>
            </svg>
        }
    }
}


