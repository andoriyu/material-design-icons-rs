
pub struct IconSynagogue {
  props: crate::Props,
}

impl yew::Component for IconSynagogue {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M6,8.94V21h4l0-4.89c0-1,0.68-1.92,1.66-2.08C12.92,13.82,14,14.79,14,16v5h4V8.94c0-0.59-0.26-1.16-0.72-1.54l-4-3.33 c-0.74-0.62-1.82-0.62-2.56,0l-4,3.33C6.26,7.78,6,8.34,6,8.94z M13.5,10c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5 s0.67-1.5,1.5-1.5S13.5,9.17,13.5,10z"/><path d="M3,5C1.9,5,1,5.9,1,7v1h4V7C5,5.9,4.1,5,3,5z"/><path d="M3,21h2V9H1v10C1,20.1,1.9,21,3,21z"/><path d="M21,5c-1.1,0-2,0.9-2,2v1h4V7C23,5.9,22.1,5,21,5z"/><path d="M19,21h2c1.1,0,2-0.9,2-2V9h-4V21z"/></g></g></svg>
            </svg>
        }
    }
}

