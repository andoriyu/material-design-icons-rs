
pub struct IconMotionPhotosPause {
  props: crate::Props,
}

impl yew::Component for IconMotionPhotosPause {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M2.88,7.88l1.54,1.54C4.15,10.23,4,11.1,4,12c0,4.41,3.59,8,8,8s8-3.59,8-8s-3.59-8-8-8c-0.9,0-1.77,0.15-2.58,0.42 L7.89,2.89C9.15,2.32,10.54,2,12,2c5.52,0,10,4.48,10,10s-4.48,10-10,10S2,17.52,2,12C2,10.53,2.32,9.14,2.88,7.88z M7,5.5 C7,6.33,6.33,7,5.5,7S4,6.33,4,5.5S4.67,4,5.5,4S7,4.67,7,5.5z M11,15H9V9h2V15z M15,15h-2V9h2V15z"/></svg>
            </svg>
        }
    }
}

