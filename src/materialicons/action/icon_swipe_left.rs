
pub struct IconSwipeLeft {
  props: crate::Props,
}

impl yew::Component for IconSwipeLeft {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M19.98,16.82l-0.63,4.46C19.21,22.27,18.36,23,17.37,23h-6.16c-0.53,0-1.29-0.21-1.66-0.59L5,17.62l0.83-0.84 c0.24-0.24,0.58-0.35,0.92-0.28L10,17.24V6.5C10,5.67,10.67,5,11.5,5S13,5.67,13,6.5v6h0.91c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04 C19.66,15.14,20.1,15.97,19.98,16.82z M12,2.5c4.74,0,7.67,2.52,8.43,4.5H22c-0.73-2.88-4.51-6-10-6C8.78,1,5.82,2.13,3.5,4.02V2H2 v5h5V5.5H4.09C6.21,3.64,8.97,2.5,12,2.5z"/></g></svg>
            </svg>
        }
    }
}


