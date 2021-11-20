
pub struct IconRemoveModerator {
  props: crate::Props,
}

impl yew::Component for IconRemoveModerator {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M12,4.14l6,2.25v4.7c0,1.19-0.23,2.36-0.64,3.44l1.51,1.51c0.72-1.53,1.13-3.22,1.13-4.95V5l-8-3L6.78,3.96l1.55,1.55 L12,4.14z M2.81,2.81L1.39,4.22L4,6.83v4.26c0,5.05,3.41,9.76,8,10.91c1.72-0.43,3.28-1.36,4.55-2.62l3.23,3.23l1.41-1.41 L2.81,2.81z M12,19.92c-3.45-1.13-6-4.82-6-8.83V8.83l9.14,9.14C14.24,18.85,13.17,19.54,12,19.92z"/></g></svg>
            </svg>
        }
    }
}


