
pub struct IconLensBlur {
  props: crate::Props,
}

impl yew::Component for IconLensBlur {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><path d="M6,13c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S6.55,13,6,13z M6,17c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1 S6.55,17,6,17z M6,9c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S6.55,9,6,9z M3,9.5c-0.28,0-0.5,0.22-0.5,0.5s0.22,0.5,0.5,0.5 s0.5-0.22,0.5-0.5S3.28,9.5,3,9.5z M6,5C5.45,5,5,5.45,5,6s0.45,1,1,1s1-0.45,1-1S6.55,5,6,5z M21,10.5c0.28,0,0.5-0.22,0.5-0.5 S21.28,9.5,21,9.5s-0.5,0.22-0.5,0.5S20.72,10.5,21,10.5z M14,7c0.55,0,1-0.45,1-1s-0.45-1-1-1s-1,0.45-1,1S13.45,7,14,7z M14,3.5 c0.28,0,0.5-0.22,0.5-0.5S14.28,2.5,14,2.5S13.5,2.72,13.5,3S13.72,3.5,14,3.5z M3,13.5c-0.28,0-0.5,0.22-0.5,0.5 s0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5S3.28,13.5,3,13.5z M10,20.5c-0.28,0-0.5,0.22-0.5,0.5s0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5 S10.28,20.5,10,20.5z M10,3.5c0.28,0,0.5-0.22,0.5-0.5S10.28,2.5,10,2.5S9.5,2.72,9.5,3S9.72,3.5,10,3.5z M10,7c0.55,0,1-0.45,1-1 s-0.45-1-1-1S9,5.45,9,6S9.45,7,10,7z M10,12.5c-0.83,0-1.5,0.67-1.5,1.5s0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5S10.83,12.5,10,12.5z M18,13c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S18.55,13,18,13z M18,17c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1 S18.55,17,18,17z M18,9c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S18.55,9,18,9z M18,5c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1 S18.55,5,18,5z M21,13.5c-0.28,0-0.5,0.22-0.5,0.5s0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5S21.28,13.5,21,13.5z M14,17 c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S14.55,17,14,17z M14,20.5c-0.28,0-0.5,0.22-0.5,0.5s0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5 S14.28,20.5,14,20.5z M10,8.5c-0.83,0-1.5,0.67-1.5,1.5s0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5S10.83,8.5,10,8.5z M10,17 c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S10.55,17,10,17z M14,12.5c-0.83,0-1.5,0.67-1.5,1.5s0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5 S14.83,12.5,14,12.5z M14,8.5c-0.83,0-1.5,0.67-1.5,1.5s0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5S14.83,8.5,14,8.5z"/></g></g></svg>
            </svg>
        }
    }
}


