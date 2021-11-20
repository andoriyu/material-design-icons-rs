
pub struct IconExploreOff {
  props: crate::Props,
}

impl yew::Component for IconExploreOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 20c1.48 0 2.86-.41 4.06-1.12l-3.98-3.98-5.58 2.6 2.59-5.58-3.97-3.98C4.41 9.14 4 10.52 4 12c0 4.41 3.59 8 8 8zm0-16c-1.48 0-2.86.41-4.06 1.12l3.98 3.98 5.58-2.6-2.59 5.58 3.98 3.98c.7-1.2 1.11-2.58 1.11-4.06 0-4.41-3.59-8-8-8z" opacity=".3"/><path d="M17.5 6.5l-5.58 2.59 2.99 2.99zM2.1 4.93l1.56 1.56C2.61 8.07 2 9.96 2 12c0 5.52 4.48 10 10 10 2.04 0 3.93-.61 5.51-1.66l1.56 1.56 1.41-1.41L3.51 3.51 2.1 4.93zm3.02 3.01l3.98 3.98-2.6 5.58 5.58-2.59 3.98 3.98c-1.2.7-2.58 1.11-4.06 1.11-4.41 0-8-3.59-8-8 0-1.48.41-2.86 1.12-4.06zM12 4c4.41 0 8 3.59 8 8 0 1.48-.41 2.86-1.12 4.06l1.46 1.46C21.39 15.93 22 14.04 22 12c0-5.52-4.48-10-10-10-2.04 0-3.93.61-5.51 1.66l1.46 1.46C9.14 4.41 10.52 4 12 4z"/></svg>
            </svg>
        }
    }
}


