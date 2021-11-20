
pub struct IconPinDrop {
  props: crate::Props,
}

impl yew::Component for IconPinDrop {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" version="1" viewBox="0 0 24 24" width="24"><path d="M12,3C9.19,3,6,5.11,6,9.13c0,2.68,2,5.49,6,8.44c4-2.95,6-5.77,6-8.44C18,5.11,14.81,3,12,3z" fill-opacity=".3"/><path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,4c1.93,0,5,1.4,5,5.15c0,2.16-1.72,4.67-5,7.32c-3.28-2.65-5-5.17-5-7.32C7,5.4,10.07,4,12,4 M12,2 C8.73,2,5,4.46,5,9.15c0,3.12,2.33,6.41,7,9.85c4.67-3.44,7-6.73,7-9.85C19,4.46,15.27,2,12,2z"/><path d="M12,7c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S13.1,7,12,7z M5,20h14v2H5V20z"/></svg>
            </svg>
        }
    }
}


