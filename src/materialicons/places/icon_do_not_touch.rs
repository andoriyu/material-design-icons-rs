
pub struct IconDoNotTouch {
  props: crate::Props,
}

impl yew::Component for IconDoNotTouch {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M13,10.17l-2.5-2.5V2.25C10.5,1.56,11.06,1,11.75,1S13,1.56,13,2.25V10.17z M20,12.75V11V5.25C20,4.56,19.44,4,18.75,4 S17.5,4.56,17.5,5.25V11h-1V3.25C16.5,2.56,15.94,2,15.25,2S14,2.56,14,3.25v7.92l6,6V12.75z M9.5,4.25C9.5,3.56,8.94,3,8.25,3 c-0.67,0-1.2,0.53-1.24,1.18L9.5,6.67V4.25z M13,10.17l-2.5-2.5V2.25C10.5,1.56,11.06,1,11.75,1S13,1.56,13,2.25V10.17z M20,12.75 V11V5.25C20,4.56,19.44,4,18.75,4S17.5,4.56,17.5,5.25V11h-1V3.25C16.5,2.56,15.94,2,15.25,2S14,2.56,14,3.25v7.92l6,6V12.75z M9.5,4.25C9.5,3.56,8.94,3,8.25,3c-0.67,0-1.2,0.53-1.24,1.18L9.5,6.67V4.25z M21.19,21.19L2.81,2.81L1.39,4.22l5.63,5.63L7,9.83 v4.3c-1.11-0.64-2.58-1.47-2.6-1.48c-0.17-0.09-0.34-0.14-0.54-0.14c-0.26,0-0.5,0.09-0.7,0.26C3.12,12.78,2,13.88,2,13.88 l6.8,7.18c0.57,0.6,1.35,0.94,2.18,0.94H17c0.62,0,1.18-0.19,1.65-0.52l-0.02-0.02l1.15,1.15L21.19,21.19z"/></g></svg>
            </svg>
        }
    }
}


