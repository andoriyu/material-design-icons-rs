
pub struct IconRemoveDone {
  props: crate::Props,
}

impl yew::Component for IconRemoveDone {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M4.14,2.69L4.14,2.69c-0.39,0.39-0.39,1.02,0,1.41l9.67,9.67l-1.41,1.41l-3.54-3.53c-0.39-0.39-1.02-0.39-1.41,0l0,0 c-0.39,0.39-0.39,1.02,0,1.41l4.24,4.24c0.39,0.39,1.02,0.39,1.41,0l2.12-2.12l5.89,5.89c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L5.55,2.69C5.16,2.3,4.53,2.3,4.14,2.69z M18.05,12.36l4.24-4.24c0.39-0.39,0.39-1.03-0.01-1.42l0,0 c-0.39-0.38-1.02-0.38-1.41,0.01l-4.24,4.24L18.05,12.36z M16.64,6.7L16.64,6.7c-0.39-0.39-1.02-0.39-1.41,0l-1.42,1.42l1.41,1.41 l1.42-1.42C17.03,7.72,17.03,7.09,16.64,6.7z M1.79,13.06l4.95,4.95l1.41-1.41L3.2,11.65c-0.39-0.39-1.02-0.39-1.41,0l0,0 C1.4,12.04,1.4,12.67,1.79,13.06z"/></g></svg>
            </svg>
        }
    }
}


