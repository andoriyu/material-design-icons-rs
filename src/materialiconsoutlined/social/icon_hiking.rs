
pub struct IconHiking {
  props: crate::Props,
}

impl yew::Component for IconHiking {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M13.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S12.4,5.5,13.5,5.5z M17.5,10.78c-1.23-0.37-2.22-1.17-2.8-2.18l-1-1.6 c-0.41-0.65-1.11-1-1.84-1c-0.78,0-1.59,0.5-1.78,1.44S7,23,7,23h2.1l1.8-8l2.1,2v6h2v-7.5l-2.1-2l0.6-3c1,1.15,2.41,2.01,4,2.34V23 H19V9h-1.5L17.5,10.78z M7.43,13.13l-2.12-0.41c-0.54-0.11-0.9-0.63-0.79-1.17l0.76-3.93c0.21-1.08,1.26-1.79,2.34-1.58l1.16,0.23 L7.43,13.13z"/></svg>
            </svg>
        }
    }
}


