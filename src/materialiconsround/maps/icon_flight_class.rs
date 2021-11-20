
pub struct IconFlightClass {
  props: crate::Props,
}

impl yew::Component for IconFlightClass {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24" y="0"/><path d="M16,4h-2c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2V6C18,4.9,17.1,4,16,4z M6,4c0.55,0,1,0.45,1,1v3l2.5,8H17 c0.55,0,1,0.45,1,1s-0.45,1-1,1H9.49c-0.88,0-1.66-0.58-1.92-1.43L5.08,8.28C5.03,8.09,5,7.9,5,7.71V5C5,4.45,5.45,4,6,4z M18,20 c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1s0.45-1,1-1h8C17.55,19,18,19.45,18,20z"/></svg>
            </svg>
        }
    }
}


