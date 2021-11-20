
pub struct IconHorizontalDistribute {
  props: crate::Props,
}

impl yew::Component for IconHorizontalDistribute {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M3,22L3,22c-0.55,0-1-0.45-1-1V3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v18C4,21.55,3.55,22,3,22z M21,2L21,2 c-0.55,0-1,0.45-1,1v18c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V3C22,2.45,21.55,2,21,2z M12,7L12,7c-0.83,0-1.5,0.67-1.5,1.5v7 c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5v-7C13.5,7.67,12.83,7,12,7z"/></svg>
            </svg>
        }
    }
}


