
pub struct IconMoveUp {
  props: crate::Props,
}

impl yew::Component for IconMoveUp {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M3.01,13.28c-0.14-2.57,1.66-4.73,4.07-5.18L6.29,8.88c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0 l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41L7.71,3.7c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l0.88,0.88l0,0.06 c-3.64,0.43-6.43,3.65-6.15,7.47C1.29,17.22,4.55,20,8.26,20H10c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H8.22 C5.52,18,3.15,15.96,3.01,13.28z"/><path d="M13,15v3c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h-5C13.9,13,13,13.9,13,15z M20,18h-5v-3h5V18z"/><path d="M20,4h-5c-1.1,0-2,0.9-2,2v3c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z"/></g></g></svg>
            </svg>
        }
    }
}


