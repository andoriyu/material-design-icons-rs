
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M20,11v2h-2L15,3V1h-2v2h-2.03V1h-2v2.12L6,13H4v-2H2v11h9v-5h2v5h9V11H20z M15.31,11H8.69l0.6-2h5.42L15.31,11z M14.11,7 H9.89l0.6-2h3.02L14.11,7z M20,20h-5v-5H9v5H4v-5h3.49l0.6-2h7.82l0.6,2H20V20z"/></g></svg>
            </svg>
        }
    }
}


