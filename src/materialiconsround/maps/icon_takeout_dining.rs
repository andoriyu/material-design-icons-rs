
pub struct IconTakeoutDining {
  props: crate::Props,
}

impl yew::Component for IconTakeoutDining {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M21.29,6.75c-0.39-0.39-1.01-0.39-1.4,0L19,7.63l0.03-0.56l-3.46-3.48C15.19,3.21,14.68,3,14.15,3h-4.3 C9.32,3,8.81,3.21,8.43,3.59L4.97,7.07L5,7.57L4.11,6.7C3.72,6.32,3.1,6.32,2.72,6.71L2.7,6.73C2.32,7.12,2.32,7.75,2.72,8.13 L4.66,10h14.69l1.92-1.84C21.67,7.78,21.68,7.14,21.29,6.75z"/><path d="M5.79,18.15C5.87,19.19,6.74,20,7.79,20h8.43c1.05,0,1.92-0.81,1.99-1.85l0.49-6.6H5.3L5.79,18.15z"/></g></g></svg>
            </svg>
        }
    }
}


