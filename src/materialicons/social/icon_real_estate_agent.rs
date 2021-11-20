
pub struct IconRealEstateAgent {
  props: crate::Props,
}

impl yew::Component for IconRealEstateAgent {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M1,22h4V11H1V22z M20,17h-7l-2.09-0.73l0.33-0.94L13,16h2.82c0.65,0,1.18-0.53,1.18-1.18l0,0c0-0.49-0.31-0.93-0.77-1.11 L8.97,11H7v9.02L14,22l8-3l0,0C21.99,17.9,21.11,17,20,17z M14,1.5l-7,5V9h2l8.14,3.26C18.26,12.71,19,13.79,19,15h2V6.5L14,1.5z M13.5,10h-1V9h1V10z M13.5,8h-1V7h1V8z M15.5,10h-1V9h1V10z M15.5,8h-1V7h1V8z"/></svg>
            </svg>
        }
    }
}


