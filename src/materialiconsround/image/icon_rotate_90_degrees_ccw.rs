
pub struct IconRotate90DegreesCcw {
  props: crate::Props,
}

impl yew::Component for IconRotate90DegreesCcw {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M5.93 7.83l-3.65 3.66c-.78.78-.78 2.05 0 2.83l3.66 3.66c.78.78 2.05.78 2.83 0l3.66-3.65c.78-.78.78-2.05 0-2.83L8.76 7.82c-.79-.78-2.05-.78-2.83.01zM4.4 12.19l2.25-2.25c.39-.39 1.02-.39 1.42 0l2.24 2.24c.39.39.39 1.02 0 1.41l-2.25 2.25c-.39.39-1.02.39-1.42 0L4.4 13.61c-.39-.39-.39-1.03 0-1.42zm14.96-5.55C17.61 4.88 15.3 4 13 4v-.83c0-.89-1.08-1.34-1.71-.71L9.47 4.29c-.39.39-.39 1.02 0 1.41l1.83 1.83c.62.63 1.7.19 1.7-.7V6c2.02 0 4.03.86 5.45 2.61 2.05 2.52 2.05 6.27 0 8.79C17.03 19.14 15.02 20 13 20c-.78 0-1.55-.13-2.29-.39-.36-.12-.75-.01-1.02.26-.5.5-.34 1.39.34 1.62.96.34 1.96.51 2.97.51 2.3 0 4.61-.88 6.36-2.64 3.52-3.51 3.52-9.21 0-12.72z"/></svg>
            </svg>
        }
    }
}


