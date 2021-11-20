
pub struct IconSquareFoot {
  props: crate::Props,
}

impl yew::Component for IconSquareFoot {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M17.66,17.66l-0.71,0.71c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71l-1.94-1.94l-0.71,0.71 c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71l-1.94-1.94l-0.71,0.71c-0.2,0.2-0.51,0.2-0.71,0l0,0 c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71L9.7,9.7l-0.71,0.71c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71 L7.05,7.05L6.34,7.76c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71L4.85,4.85C4.54,4.54,4,4.76,4,5.21V18 c0,1.1,0.9,2,2,2h12.79c0.45,0,0.67-0.54,0.35-0.85L17.66,17.66z M7,16v-4.76L12.76,17H8C7.45,17,7,16.55,7,16z"/></g></g></svg>
            </svg>
        }
    }
}


