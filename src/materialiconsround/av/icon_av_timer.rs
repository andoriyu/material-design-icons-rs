
pub struct IconAvTimer {
  props: crate::Props,
}

impl yew::Component for IconAvTimer {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><circle cx="12" cy="17" r="1"/><circle cx="7" cy="12" r="1"/><circle cx="17" cy="12" r="1"/><path d="M12 3c-.55 0-1 .45-1 1v2c0 .55.45 1 1 1s1-.45 1-1v-.92c3.31.48 5.87 3.25 6 6.66.14 3.85-3.03 7.2-6.88 7.26C8.19 19.06 5 15.91 5 12c0-1.68.59-3.22 1.58-4.42l4.71 4.72c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L7.26 5.46c-.38-.38-1-.39-1.4-.02C4.1 7.07 3 9.4 3 12c0 5.04 4.14 9.12 9.21 9 4.7-.11 8.63-4.01 8.78-8.71C21.16 7.19 17.07 3 12 3z"/></svg>
            </svg>
        }
    }
}


