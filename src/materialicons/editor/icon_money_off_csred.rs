
pub struct IconMoneyOffCsred {
  props: crate::Props,
}

impl yew::Component for IconMoneyOffCsred {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M10.53,7.43c0.42-0.31,0.93-0.47,1.54-0.47s1.11,0.16,1.5,0.49c0.39,0.32,0.65,0.7,0.79,1.12l1.89-0.8 c-0.24-0.71-0.71-1.35-1.4-1.92c-0.5-0.4-1.12-0.65-1.85-0.77V3h-2v2.11c-0.41,0.08-0.79,0.21-1.14,0.39 c-0.35,0.18-0.64,0.39-0.9,0.63l1.43,1.43C10.43,7.52,10.48,7.47,10.53,7.43z M2.81,2.81L1.39,4.22l12.35,12.35 C13.31,16.85,12.79,17,12.19,17c-0.71,0-1.32-0.23-1.83-0.7c-0.5-0.47-0.86-1.07-1.06-1.81l-1.98,0.8 c0.34,1.17,0.95,2.08,1.83,2.73c0.57,0.42,1.19,0.68,1.85,0.83V21h2v-2.08c0.44-0.07,0.87-0.17,1.29-0.35 c0.34-0.14,0.64-0.32,0.92-0.53l4.57,4.57l1.41-1.41L2.81,2.81z"/></g></svg>
            </svg>
        }
    }
}


