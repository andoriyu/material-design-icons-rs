
pub struct IconTextRotationAngledown {
  props: crate::Props,
}

impl yew::Component for IconTextRotationAngledown {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0zm0 0h24v24H0V0zm0 0h24v24H0V0z" fill="none"/><path d="M15 21v-4.24l-1.41 1.41-9.2-9.19-1.41 1.41 9.19 9.19L10.76 21H15zM11.25 8.48l3.54 3.54-.92 2.19 1.48 1.48 4.42-11.14-1.06-1.05L7.57 7.92 9.06 9.4l2.19-.92zm6.59-3.05l-2.23 4.87-2.64-2.64 4.87-2.23z"/></svg>
            </svg>
        }
    }
}


