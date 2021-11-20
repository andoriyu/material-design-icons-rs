
pub struct IconHive {
  props: crate::Props,
}

impl yew::Component for IconHive {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M14.09,7.51l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C13.91,2.18,13.58,2,13.23,2h-2.45c-0.35,0-0.68,0.18-0.86,0.49 l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2C10.09,7.82,10.42,8,10.77,8h2.45C13.58,8,13.91,7.82,14.09,7.51z"/><path d="M9.91,9.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49h2.46c0.35,0,0.68-0.18,0.86-0.49 l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C13.91,9.18,13.58,9,13.23,9h-2.46C10.42,9,10.09,9.18,9.91,9.49z"/><path d="M17.01,11.51h2.45c0.35,0,0.68-0.18,0.86-0.49l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2c-0.18-0.3-0.51-0.49-0.86-0.49 h-2.45c-0.35,0-0.68,0.18-0.86,0.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2C16.34,11.33,16.66,11.51,17.01,11.51z"/><path d="M19.47,12.51h-2.46c-0.35,0-0.68,0.18-0.86,0.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49 h2.46c0.35,0,0.68-0.18,0.86-0.49l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C20.15,12.7,19.82,12.51,19.47,12.51z"/><path d="M7.84,11.03l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C7.66,5.7,7.34,5.51,6.99,5.51H4.53C4.18,5.51,3.85,5.7,3.67,6 l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49h2.45C7.34,11.51,7.66,11.33,7.84,11.03z"/><path d="M6.99,12.51H4.53c-0.35,0-0.68,0.18-0.86,0.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49 h2.46c0.35,0,0.68-0.18,0.86-0.49l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C7.66,12.7,7.34,12.51,6.99,12.51z"/><path d="M9.91,16.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49h2.46c0.35,0,0.68-0.18,0.86-0.49 l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2c-0.18-0.3-0.51-0.49-0.86-0.49h-2.46C10.42,16,10.09,16.18,9.91,16.49z"/></g></g></svg>
            </svg>
        }
    }
}


