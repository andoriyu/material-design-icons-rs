
pub struct IconFilePresent {
  props: crate::Props,
}

impl yew::Component for IconFilePresent {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M13.17,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8.83c0-0.53-0.21-1.04-0.59-1.41l-4.83-4.83 C14.21,2.21,13.7,2,13.17,2z M16,15c0,2.34-2.01,4.21-4.39,3.98C9.53,18.78,8,16.92,8,14.83l0-5.19c0-1.31,0.94-2.5,2.24-2.63 C11.74,6.86,13,8.03,13,9.5V14c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V9.5C11,9.22,10.78,9,10.5,9S10,9.22,10,9.5v5.39 c0,1,0.68,1.92,1.66,2.08C12.92,17.18,14,16.21,14,15v-3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1V15z M14,7V4l4,4h-3 C14.45,8,14,7.55,14,7z"/></g></svg>
            </svg>
        }
    }
}


