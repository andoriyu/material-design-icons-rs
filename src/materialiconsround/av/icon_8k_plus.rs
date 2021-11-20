
pub struct Icon8kPlus {
  props: crate::Props,
}

impl yew::Component for Icon8kPlus {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M10,14c0,0.55-0.45,1-1,1H7 c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h2c0.55,0,1,0.45,1,1V14z M14.59,15c-0.22,0-0.42-0.1-0.55-0.27l-1.54-1.98v1.55 c0,0.39-0.31,0.7-0.7,0.7H11.7c-0.39,0-0.7-0.31-0.7-0.7V9.7C11,9.31,11.31,9,11.7,9h0.09c0.39,0,0.7,0.31,0.7,0.7v1.55l1.54-1.98 C14.17,9.1,14.38,9,14.59,9c0.58,0,0.91,0.66,0.56,1.12L13.75,12l1.41,1.88C15.5,14.34,15.17,15,14.59,15z M19,12.5h-1.5V14h-1 v-1.5H15v-1h1.5V10h1v1.5H19V12.5z"/><rect height="1.5" width="1" x="7.5" y="12.5"/><rect height="1.5" width="1" x="7.5" y="10"/></g></g></svg>
            </svg>
        }
    }
}


