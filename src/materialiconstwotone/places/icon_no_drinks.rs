
pub struct IconNoDrinks {
  props: crate::Props,
}

impl yew::Component for IconNoDrinks {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><polygon opacity=".3" points="14.77,9 11.83,9 13.38,10.56"/><path d="M21.19,21.19L2.81,2.81L1.39,4.22l8.23,8.23L11,14v5H6v2h12v-0.17l1.78,1.78L21.19,21.19z M13,19v-3.17L16.17,19H13z M7.83,5l-2-2H21v2l-6.2,6.97l-1.42-1.42L14.77,9h-2.94l-2-2h6.74l1.78-2H7.83z"/></g></svg>
            </svg>
        }
    }
}


