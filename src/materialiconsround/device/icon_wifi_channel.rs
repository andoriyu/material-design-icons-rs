
pub struct IconWifiChannel {
  props: crate::Props,
}

impl yew::Component for IconWifiChannel {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M4.13,21c0.5,0,0.92-0.38,0.99-0.87c0.65-4.89,1.95-9.01,2.88-10c0.91,0.98,2.19,5.01,2.86,9.82 c0.08,0.6,0.59,1.05,1.19,1.05c0.54,0,1.02-0.36,1.16-0.89C13.83,17.73,15.11,15,16,15c0.9,0,2.19,2.83,2.81,5.2 c0.12,0.48,0.56,0.8,1.05,0.8c0.62,0,1.12-0.52,1.09-1.14C20.75,15.89,19.81,3,16,3c-2.51,0-3.77,5.61-4.4,10.57 C10.79,10.66,9.61,8,8,8c-2.92,0-4.41,8.71-4.85,11.87C3.06,20.47,3.53,21,4.13,21z M16,13c-0.99,0-1.82,0.62-2.5,1.5 c0.57-4.77,1.54-8.62,2.5-9.44c0.97,0.81,1.91,4.67,2.49,9.43C17.81,13.62,16.98,13,16,13z"/></g></svg>
            </svg>
        }
    }
}


