
pub struct Icon2kPlus {
  props: crate::Props,
}

impl yew::Component for Icon2kPlus {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M10,11.5c0,0.55-0.45,1-1,1H7.5v1 h1.75c0.41,0,0.75,0.34,0.75,0.75v0C10,14.66,9.66,15,9.25,15H7c-0.55,0-1-0.45-1-1v-1.5c0-0.55,0.45-1,1-1h1.5v-1H6.75 C6.34,10.5,6,10.16,6,9.75v0C6,9.34,6.34,9,6.75,9H9c0.55,0,1,0.45,1,1V11.5z M14.04,14.73l-1.54-1.98v1.5 c0,0.41-0.34,0.75-0.75,0.75h0C11.34,15,11,14.66,11,14.25v-4.5C11,9.34,11.34,9,11.75,9h0c0.41,0,0.75,0.34,0.75,0.75v1.5 l1.54-1.98C14.17,9.1,14.38,9,14.59,9h0c0.58,0,0.91,0.66,0.56,1.12L13.75,12l1.41,1.88C15.5,14.34,15.17,15,14.59,15h0 C14.38,15,14.17,14.9,14.04,14.73z M18.5,12.5h-1v1c0,0.28-0.22,0.5-0.5,0.5l0,0c-0.28,0-0.5-0.22-0.5-0.5v-1h-1 c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5h1v-1c0-0.28,0.22-0.5,0.5-0.5l0,0c0.28,0,0.5,0.22,0.5,0.5v1h1 c0.28,0,0.5,0.22,0.5,0.5v0C19,12.28,18.78,12.5,18.5,12.5z"/></g></svg>
            </svg>
        }
    }
}


