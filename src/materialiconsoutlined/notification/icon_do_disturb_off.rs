
pub struct IconDoDisturbOff {
  props: crate::Props,
}

impl yew::Component for IconDoDisturbOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 4c4.41 0 8 3.59 8 8 0 1.41-.37 2.73-1.01 3.88l1.46 1.46C21.43 15.79 22 13.96 22 12c0-5.52-4.48-10-10-10-1.96 0-3.79.57-5.33 1.55l1.46 1.46C9.27 4.37 10.59 4 12 4zm5 7h-2.88l2 2H17zM2.41 2.13L1 3.54l2.78 2.78C2.66 7.93 2 9.89 2 12c0 5.52 4.48 10 10 10 2.11 0 4.07-.66 5.68-1.78L20.46 23l1.41-1.41L2.41 2.13zM12 20c-4.41 0-8-3.59-8-8 0-1.56.45-3 1.23-4.23L8.46 11H7v2h3.46l5.77 5.77C15 19.55 13.56 20 12 20z"/></svg>
            </svg>
        }
    }
}


