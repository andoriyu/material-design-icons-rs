
pub struct IconLocalConvenienceStore {
  props: crate::Props,
}

impl yew::Component for IconLocalConvenienceStore {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M14 14h2v4h4V9h-3V6H7v3H4v9h4v-4h6zm-1-7h1v2h1V7h1v5h-1v-2h-2V7zM8 9h2V8H8V7h3v3H9v1h2v1H8V9z" opacity=".3"/><path d="M10 16h4v4h8V7h-3V4H5v3H2v13h8v-4zm-2 0v2H4V9h3V6h10v3h3v9h-4v-4H8v2zm3-5H9v-1h2V7H8v1h2v1H8v3h3zm4 1h1V7h-1v2h-1V7h-1v3h2z"/></svg>
            </svg>
        }
    }
}


