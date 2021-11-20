
pub struct IconUTurnLeft {
  props: crate::Props,
}

impl yew::Component for IconUTurnLeft {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M3.71,12.29c0.39-0.39,1.02-0.39,1.41,0L6,13.17V9c0-3.31,2.69-6,6-6s6,2.69,6,6v11c0,0.55-0.45,1-1,1s-1-0.45-1-1V9 c0-2.21-1.79-4-4-4S8,6.79,8,9v4.17l0.88-0.88c0.39-0.39,1.02-0.39,1.41,0c0.39,0.39,0.39,1.02,0,1.41l-2.59,2.59 c-0.39,0.39-1.02,0.39-1.41,0l-2.59-2.59C3.32,13.32,3.32,12.68,3.71,12.29z"/></g></svg>
            </svg>
        }
    }
}


