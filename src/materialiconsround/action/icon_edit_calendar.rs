
pub struct IconEditCalendar {
  props: crate::Props,
}

impl yew::Component for IconEditCalendar {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M12,22H5c-1.11,0-2-0.9-2-2L3.01,6c0-1.1,0.88-2,1.99-2h1V3c0-0.55,0.45-1,1-1s1,0.45,1,1v1h8V3c0-0.55,0.45-1,1-1 s1,0.45,1,1v1h1c1.1,0,2,0.9,2,2v6h-2v-2H5v10h7V22z M22.13,16.99l0.71-0.71c0.39-0.39,0.39-1.02,0-1.41l-0.71-0.71 c-0.39-0.39-1.02-0.39-1.41,0l-0.71,0.71L22.13,16.99z M21.42,17.7l-5.01,5.01c-0.18,0.18-0.44,0.29-0.7,0.29H14.5 c-0.28,0-0.5-0.22-0.5-0.5v-1.21c0-0.27,0.11-0.52,0.29-0.71l5.01-5.01L21.42,17.7z"/></svg>
            </svg>
        }
    }
}


