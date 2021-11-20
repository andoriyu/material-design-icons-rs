
pub struct IconTaskAlt {
  props: crate::Props,
}

impl yew::Component for IconTaskAlt {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M21.29,5.89l-10,10c-0.39,0.39-1.02,0.39-1.41,0l-2.83-2.83c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0 l2.12,2.12l9.29-9.29c0.39-0.39,1.02-0.39,1.41,0l0,0C21.68,4.87,21.68,5.5,21.29,5.89z M15.77,2.74c-1.69-0.69-3.61-0.93-5.61-0.57 C6.09,2.9,2.84,6.18,2.15,10.25C1.01,17,6.63,22.78,13.34,21.91c3.96-0.51,7.28-3.46,8.32-7.31c0.4-1.47,0.44-2.89,0.21-4.22 c-0.13-0.8-1.12-1.11-1.7-0.54v0c-0.23,0.23-0.33,0.57-0.27,0.89c0.22,1.33,0.12,2.75-0.52,4.26c-1.16,2.71-3.68,4.7-6.61,4.97 c-5.1,0.47-9.33-3.85-8.7-8.98c0.43-3.54,3.28-6.42,6.81-6.91c1.73-0.24,3.37,0.09,4.77,0.81c0.39,0.2,0.86,0.13,1.17-0.18l0,0 c0.48-0.48,0.36-1.29-0.24-1.6C16.31,2.98,16.04,2.85,15.77,2.74z"/></svg>
            </svg>
        }
    }
}


