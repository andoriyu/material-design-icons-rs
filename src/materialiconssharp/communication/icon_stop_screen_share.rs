
pub struct IconStopScreenShare {
  props: crate::Props,
}

impl yew::Component for IconStopScreenShare {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0z" fill="none"/><path d="M21.79 18l2 2H24v-2zM13 9.13V7l4 3.74-1.28 1.19 5.18 5.18L22 16V4.02H7.8l5.13 5.13c.03-.01.05-.02.07-.02zM1.11 2.98l.89.9v12.14l2 1.99L0 18v2h18.13l2.71 2.71 1.41-1.41L2.52 1.57 1.11 2.98zm7.97 7.97l1.59 1.59C9.13 12.92 7.96 13.71 7 15c.31-1.48.94-2.93 2.08-4.05z"/></svg>
            </svg>
        }
    }
}


