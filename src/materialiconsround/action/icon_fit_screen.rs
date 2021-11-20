
pub struct IconFitScreen {
  props: crate::Props,
}

impl yew::Component for IconFitScreen {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M18,4h2c1.1,0,2,0.9,2,2v2c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V6h-2c-0.55,0-1-0.45-1-1v0C17,4.45,17.45,4,18,4z M4,8 l0-2h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4C2.9,4,2,4.9,2,6l0,2c0,0.55,0.45,1,1,1h0C3.55,9,4,8.55,4,8z M20,16v2h-2 c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2c1.1,0,2-0.9,2-2v-2c0-0.55-0.45-1-1-1h0C20.45,15,20,15.45,20,16z M6,18H4v-2 c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2c0,1.1,0.9,2,2,2h2c0.55,0,1-0.45,1-1v0C7,18.45,6.55,18,6,18z M16,8H8c-1.1,0-2,0.9-2,2 v4c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-4C18,8.9,17.1,8,16,8z"/></g></svg>
            </svg>
        }
    }
}


