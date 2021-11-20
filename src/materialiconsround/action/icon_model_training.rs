
pub struct IconModelTraining {
  props: crate::Props,
}

impl yew::Component for IconModelTraining {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M15.5,13.5c0,2-2.5,3.5-2.5,5h-2c0-1.5-2.5-3-2.5-5c0-1.93,1.57-3.5,3.5-3.5h0C13.93,10,15.5,11.57,15.5,13.5z M13,19.5h-2 V20c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V19.5z M19,13c0,1.39-0.41,2.69-1.12,3.78c-0.25,0.39-0.19,0.91,0.14,1.24l0,0 c0.44,0.44,1.2,0.38,1.54-0.15C20.47,16.47,21,14.8,21,13c0-2.36-0.91-4.51-2.4-6.12c-0.39-0.42-1.05-0.43-1.45-0.03l0,0 c-0.38,0.38-0.38,0.99-0.02,1.39C18.29,9.49,19,11.16,19,13z M15.65,4.65l-2.79-2.79C12.54,1.54,12,1.76,12,2.21V4l0,0 c-4.97,0-9,4.03-9,9c0,1.8,0.53,3.47,1.44,4.88c0.34,0.53,1.1,0.59,1.54,0.15l0,0c0.33-0.33,0.39-0.84,0.14-1.23 C4.73,14.65,4.48,11.7,6.25,8.8C7.45,6.85,9.71,5.81,12,6l0,0v1.79c0,0.45,0.54,0.67,0.85,0.35l2.79-2.79 C15.84,5.16,15.84,4.84,15.65,4.65z"/></g></svg>
            </svg>
        }
    }
}


