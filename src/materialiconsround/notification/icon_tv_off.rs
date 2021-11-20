
pub struct IconTvOff {
  props: crate::Props,
}

impl yew::Component for IconTvOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M21 8v9.88l1.85 1.85c.1-.22.15-.47.15-.73V7c0-1.11-.9-2-2-2h-7.59l2.94-2.94c.2-.2.2-.51 0-.71s-.51-.2-.71 0L12 4.99 8.36 1.35c-.2-.2-.51-.2-.71 0s-.2.51 0 .71L10.59 5H8.12l2 2H20c.55 0 1 .45 1 1zM3.12 2.83c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l.82.82C1.65 5.28 1 6.06 1 7v12c0 1.1.9 2 2 2h15.46l1.29 1.29c.39.39 1.02.39 1.41 0 .36-.36.37-.92.07-1.31h.03L3.12 2.83zM3 18V8c0-.55.45-1 1-1h.46l12 12H4c-.55 0-1-.45-1-1z"/></svg>
            </svg>
        }
    }
}

