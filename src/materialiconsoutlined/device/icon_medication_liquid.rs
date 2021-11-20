
pub struct IconMedicationLiquid {
  props: crate::Props,
}

impl yew::Component for IconMedicationLiquid {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><rect height="2" width="12" x="3" y="3"/><path d="M2,21h14V6H2V21z M5,12h2.5V9.5h3V12H13v3h-2.5v2.5h-3V15H5V12z"/><path d="M20,6c-1.68,0-3,1.76-3,4c0,1.77,0.83,3.22,2,3.76V21h2v-7.24c1.17-0.54,2-1.99,2-3.76C23,7.76,21.68,6,20,6z"/></g></g></svg>
            </svg>
        }
    }
}


