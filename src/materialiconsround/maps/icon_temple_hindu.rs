
pub struct IconTempleHindu {
  props: crate::Props,
}

impl yew::Component for IconTempleHindu {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><polygon points="6.6,11 17.4,11 16.5,8 7.5,8"/><path d="M20,12v1H4v-1c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v8c0,1.1,0.9,2,2,2h6v-3c0-1.1,0.9-2,2-2h0c1.1,0,2,0.9,2,2v3h6 c1.1,0,2-0.9,2-2v-8c0-0.55-0.45-1-1-1h0C20.45,11,20,11.45,20,12z"/><path d="M15.9,6L15,3V2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1h-2.03V2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1.12L8.1,6 H15.9z"/></g></g></svg>
            </svg>
        }
    }
}


