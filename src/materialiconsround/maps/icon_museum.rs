
pub struct IconMuseum {
  props: crate::Props,
}

impl yew::Component for IconMuseum {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M21.5,11c0.28,0,0.5-0.22,0.5-0.5V9.26c0-0.16-0.08-0.32-0.21-0.41L12.57,2.4c-0.34-0.24-0.8-0.24-1.15,0L2.21,8.85 C2.08,8.94,2,9.1,2,9.26v1.24C2,10.78,2.22,11,2.5,11H4v9H3c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1 c0-0.55-0.45-1-1-1h-1v-9H21.5z M16,17c0,0.55-0.45,1-1,1s-1-0.45-1-1v-3l-1.17,1.75c-0.4,0.59-1.27,0.59-1.66,0L10,14v3 c0,0.55-0.45,1-1,1s-1-0.45-1-1v-4.7C8,11.58,8.58,11,9.3,11h0c0.43,0,0.84,0.22,1.08,0.58L12,14l1.61-2.42 C13.86,11.22,14.26,11,14.7,11h0c0.72,0,1.3,0.58,1.3,1.3V17z"/></g></svg>
            </svg>
        }
    }
}


