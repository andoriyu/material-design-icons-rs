
pub struct IconSsidChart {
  props: crate::Props,
}

impl yew::Component for IconSsidChart {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M3,9.03c0-0.32,0.15-0.62,0.41-0.81L7.14,5.5c0.4-0.29,0.95-0.25,1.3,0.1l3.78,3.78l7.2-5.23C20.07,3.67,21,4.14,21,4.96 c0,0.32-0.15,0.62-0.41,0.81l-7.9,5.73c-0.4,0.29-0.95,0.25-1.29-0.1L7.62,7.62L4.59,9.84C3.93,10.32,3,9.85,3,9.03z M21,16 c0-0.55-0.45-1-1-1h-3.35c-0.23,0-0.45,0.08-0.62,0.22l-3.9,3.12L6.6,12.99c-0.35-0.34-0.88-0.38-1.27-0.1l-1.9,1.35 C3.16,14.43,3,14.74,3,15.06c0,0.81,0.92,1.29,1.58,0.81L5.8,15l5.57,5.39c0.36,0.35,0.93,0.38,1.32,0.06L17,17h3 C20.55,17,21,16.55,21,16z"/></g></svg>
            </svg>
        }
    }
}


