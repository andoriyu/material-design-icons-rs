
pub struct IconWbIncandescent {
  props: crate::Props,
}

impl yew::Component for IconWbIncandescent {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M14 8.59l-1-.58V4.05h-2v3.96l-1 .58c-1.24.72-2 2.04-2 3.46 0 2.21 1.79 4 4 4s4-1.79 4-4c0-1.42-.77-2.74-2-3.46z" opacity=".3"/><path d="M3.55 19.09l1.41 1.41 1.79-1.8-1.41-1.41zM11 20h2v3h-2zM1 11h3v2H1zm14-4.14V2.05H9v4.81C7.21 7.9 6 9.83 6 12.05c0 3.31 2.69 6 6 6s6-2.69 6-6c0-2.22-1.21-4.15-3-5.19zm-3 9.19c-2.21 0-4-1.79-4-4 0-1.42.77-2.74 2-3.46l1-.58V4.05h2v3.96l1 .58c1.24.72 2 2.04 2 3.46 0 2.21-1.79 4-4 4zM20 11h3v2h-3zm-2.76 7.71l1.79 1.8 1.41-1.41-1.8-1.79z"/></svg>
            </svg>
        }
    }
}


