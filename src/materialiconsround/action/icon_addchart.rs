
pub struct IconAddchart {
  props: crate::Props,
}

impl yew::Component for IconAddchart {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><path d="M11,10c0-0.55,0.45-1,1-1s1,0.45,1,1v7h-2V10z M20,13c-0.55,0-1,0.45-1,1v5H5V5h5c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H5 C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-5C21,13.45,20.55,13,20,13z M21,5h-2V3c0-0.55-0.45-1-1-1s-1,0.45-1,1v2 h-2c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V7h2c0.55,0,1-0.45,1-1C22,5.45,21.55,5,21,5z M16,13 c-0.55,0-1,0.45-1,1v3h2v-3C17,13.45,16.55,13,16,13z M7,12v5h2v-5c0-0.55-0.45-1-1-1S7,11.45,7,12z"/></g></svg>
            </svg>
        }
    }
}

