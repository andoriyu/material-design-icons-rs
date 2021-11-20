
pub struct IconNoMeals {
  props: crate::Props,
}

impl yew::Component for IconNoMeals {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M21,18.17l-2-2V14h-1c-1.1,0-2-0.9-2-2V6c0-1.49,1.6-3.32,3.76-3.85C20.39,2,21,2.48,21,3.13V18.17z M21.19,22.61 c-0.39,0.39-1.02,0.39-1.41,0l-9.76-9.76C9.69,12.94,9.36,13,9,13v8c0,0.55-0.45,1-1,1s-1-0.45-1-1v-8c-2.21,0-4-1.79-4-4V5.83 L1.39,4.22C1,3.83,1,3.2,1.39,2.81c0.39-0.39,1.02-0.39,1.41,0l18.38,18.38C21.58,21.58,21.58,22.22,21.19,22.61z M6.17,9L5,7.83V9 H6.17z M13,9V3c0-0.55-0.45-1-1-1s-1,0.45-1,1v5.17l1.85,1.85C12.94,9.69,13,9.36,13,9z M9,3c0-0.55-0.45-1-1-1S7,2.45,7,3v1.17l2,2 V3z"/></svg>
            </svg>
        }
    }
}


