
pub struct IconLocalPharmacy {
  props: crate::Props,
}

impl yew::Component for IconLocalPharmacy {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M5.11 19h13.78l-1.79-5.37-.21-.63.21-.63L18.89 7H5.11l1.79 5.37.21.63-.21.63L5.11 19zM8 12h3V9h2v3h3v2h-3v3h-2v-3H8v-2z" opacity=".3"/><path d="M3 21h18v-2l-2-6 2-6V5h-2.64l1.14-3.14L17.15 1l-1.46 4H3v2l2 6-2 6v2zm3.9-8.63L5.11 7h13.78l-1.79 5.37-.21.63.21.63L18.89 19H5.11l1.79-5.37.21-.63-.21-.63zM11 17h2v-3h3v-2h-3V9h-2v3H8v2h3z"/></svg>
            </svg>
        }
    }
}


