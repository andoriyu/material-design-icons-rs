
pub struct IconFrontHand {
  props: crate::Props,
}

impl yew::Component for IconFrontHand {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M18.5,8c-0.17,0-0.34,0.02-0.5,0.05V4.5C18,3.12,16.88,2,15.5,2c-0.19,0-0.37,0.02-0.54,0.06C14.75,0.89,13.73,0,12.5,0 c-1.06,0-1.96,0.66-2.33,1.59C9.96,1.53,9.73,1.5,9.5,1.5C8.12,1.5,7,2.62,7,4v0.55C6.84,4.52,6.67,4.5,6.5,4.5C5.12,4.5,4,5.62,4,7 v8.5c0,4.69,3.81,8.5,8.5,8.5s8.5-3.81,8.5-8.5v-5C21,9.12,19.88,8,18.5,8z M19,15.5c0,3.59-2.91,6.5-6.5,6.5S6,19.09,6,15.5V7 c0-0.28,0.22-0.5,0.5-0.5S7,6.72,7,7v5h2V4c0-0.28,0.22-0.5,0.5-0.5S10,3.72,10,4v7h2V2.5C12,2.22,12.22,2,12.5,2S13,2.22,13,2.5V11 h2V4.5C15,4.22,15.22,4,15.5,4S16,4.22,16,4.5v8.92c-1.77,0.77-3,2.53-3,4.58h2c0-1.66,1.34-3,3-3v-4.5c0-0.28,0.22-0.5,0.5-0.5 s0.5,0.22,0.5,0.5V15.5z"/></svg>
            </svg>
        }
    }
}


