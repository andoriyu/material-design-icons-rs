
pub struct IconTextFields {
  props: crate::Props,
}

impl yew::Component for IconTextFields {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0z" fill="none"/><path d="M2.5 5.5C2.5 6.33 3.17 7 4 7h3.5v10.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5V7H14c.83 0 1.5-.67 1.5-1.5S14.83 4 14 4H4c-.83 0-1.5.67-1.5 1.5zM20 9h-6c-.83 0-1.5.67-1.5 1.5S13.17 12 14 12h1.5v5.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5V12H20c.83 0 1.5-.67 1.5-1.5S20.83 9 20 9z"/></svg>
            </svg>
        }
    }
}


