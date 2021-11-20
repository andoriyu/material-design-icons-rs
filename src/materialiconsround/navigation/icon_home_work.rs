
pub struct IconHomeWork {
  props: crate::Props,
}

impl yew::Component for IconHomeWork {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M20,3h-8c-0.55,0-1,0.45-1,1v1.61c0,0,0,0,0.01,0.01l5,4.5C16.64,10.68,17,11.5,17,12.35V13h2v2h-2v2h2v2h-2v2h3 c0.55,0,1-0.45,1-1V4C21,3.45,20.55,3,20,3z M15,7h-2V5h2V7z M19,11h-2V9h2V11z M19,7h-2V5h2V7z"/><path d="M15,20v-7.65c0-0.28-0.12-0.55-0.33-0.74l-5-4.5C9.48,6.93,9.24,6.85,9,6.85c-0.24,0-0.48,0.09-0.67,0.26l-5,4.5 C3.12,11.79,3,12.06,3,12.35V20c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-4h4v4c0,0.55,0.45,1,1,1h2C14.55,21,15,20.55,15,20z"/></g></g></svg>
            </svg>
        }
    }
}

