
pub struct IconPlumbing {
  props: crate::Props,
}

impl yew::Component for IconPlumbing {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M19.28,4.93l-2.12-2.12c-0.78-0.78-2.05-0.78-2.83,0L11.5,5.64l2.12,2.12l2.12-2.12l3.54,3.54 C20.45,8,20.45,6.1,19.28,4.93z"/><path d="M5.49,13.77c0.59,0.59,1.54,0.59,2.12,0l2.47-2.47L7.96,9.17l-2.47,2.47C4.9,12.23,4.9,13.18,5.49,13.77L5.49,13.77z"/><path d="M15.04,7.76l-0.71,0.71l-0.71,0.71l-3.18-3.18C9.85,5.4,8.9,5.4,8.32,5.99c-0.59,0.59-0.59,1.54,0,2.12l3.18,3.18 L10.79,12l-6.36,6.36c-0.78,0.78-0.78,2.05,0,2.83c0.78,0.78,2.05,0.78,2.83,0L16.45,12c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L15.04,7.76z"/></g></g></svg>
            </svg>
        }
    }
}


