
pub struct IconSwipeVertical {
  props: crate::Props,
}

impl yew::Component for IconSwipeVertical {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M1,3.5h2.02C1.13,5.82,0,8.78,0,12s1.13,6.18,3.02,8.5H1V22h5v-5H4.5v2.91c-1.86-2.11-3-4.88-3-7.91s1.14-5.79,3-7.91V7H6 V2H1V3.5z M13.85,11.62l-2.68-5.37c-0.37-0.74-1.27-1.04-2.01-0.67C8.41,5.96,8.11,6.86,8.48,7.6l4.81,9.6L10.05,18 c-0.33,0.09-0.59,0.33-0.7,0.66L9,19.78l6.19,2.25c0.5,0.17,1.28,0.02,1.75-0.22l5.51-2.75c0.89-0.45,1.32-1.48,1-2.42l-1.43-4.27 c-0.27-0.82-1.04-1.37-1.9-1.37h-4.56c-0.31,0-0.62,0.07-0.89,0.21L13.85,11.62"/></g></svg>
            </svg>
        }
    }
}


