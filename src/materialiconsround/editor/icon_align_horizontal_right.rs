
pub struct IconAlignHorizontalRight {
  props: crate::Props,
}

impl yew::Component for IconAlignHorizontalRight {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M21,2L21,2c0.55,0,1,0.45,1,1v18c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V3C20,2.45,20.45,2,21,2z M3.5,10h13 c0.83,0,1.5-0.67,1.5-1.5v0C18,7.67,17.33,7,16.5,7h-13C2.67,7,2,7.67,2,8.5v0C2,9.33,2.67,10,3.5,10z M9.5,17h7 c0.83,0,1.5-0.67,1.5-1.5v0c0-0.83-0.67-1.5-1.5-1.5h-7C8.67,14,8,14.67,8,15.5v0C8,16.33,8.67,17,9.5,17z"/></svg>
            </svg>
        }
    }
}


