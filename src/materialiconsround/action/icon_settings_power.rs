
pub struct IconSettingsPower {
  props: crate::Props,
}

impl yew::Component for IconSettingsPower {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M8,24L8,24c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C7,23.55,7.45,24,8,24z M12,24L12,24 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C11,23.55,11.45,24,12,24z M12,2c-0.55,0-1,0.45-1,1v8 c0,0.55,0.45,1,1,1s1-0.45,1-1V3C13,2.45,12.55,2,12,2z M15.94,5.06l-0.02,0.02C15.51,5.49,15.56,6.16,16,6.54 c1.51,1.34,2.33,3.43,1.88,5.7c-0.46,2.28-2.29,4.14-4.56,4.62C9.43,17.69,6,14.74,6,11c0-1.78,0.78-3.37,2.01-4.47 c0.43-0.39,0.47-1.04,0.07-1.45L8.06,5.06C7.69,4.69,7.1,4.67,6.7,5.02c-2.01,1.77-3.12,4.53-2.56,7.52 c0.59,3.15,3.11,5.7,6.26,6.31c5.12,0.99,9.6-2.9,9.6-7.85c0-2.38-1.05-4.52-2.71-5.99C16.9,4.67,16.31,4.69,15.94,5.06z M16,24 L16,24c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v0C15,23.55,15.45,24,16,24z"/></g></svg>
            </svg>
        }
    }
}


