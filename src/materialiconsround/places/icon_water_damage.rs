
pub struct IconWaterDamage {
  props: crate::Props,
}

impl yew::Component for IconWaterDamage {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M11.33,3.6l-8.36,7.53C2.63,11.43,2.84,12,3.3,12H5v7c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1v-7h1.7 c0.46,0,0.68-0.57,0.33-0.87L12.67,3.6C12.29,3.26,11.71,3.26,11.33,3.6z M12,16c-1.1,0-2-0.9-2-2c0-0.78,0.99-2.44,1.58-3.36 c0.2-0.31,0.64-0.31,0.84,0C13.01,11.56,14,13.22,14,14C14,15.1,13.1,16,12,16z"/></svg>
            </svg>
        }
    }
}


