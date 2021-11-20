
pub struct IconFlashAuto {
  props: crate::Props,
}

impl yew::Component for IconFlashAuto {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M3 3v10c0 .55.45 1 1 1h2v7.15c0 .51.67.69.93.25l5.19-8.9c.39-.67-.09-1.5-.86-1.5H9l3.38-7.59c.29-.67-.2-1.41-.92-1.41H4c-.55 0-1 .45-1 1zm15-1c-.6 0-1.13.38-1.34.94L14.22 9.8c-.2.59.23 1.2.85 1.2.38 0 .72-.24.84-.6L16.4 9h3.2l.49 1.4c.13.36.46.6.84.6.62 0 1.05-.61.84-1.19l-2.44-6.86C19.13 2.38 18.6 2 18 2zm-1.15 5.65L18 4l1.15 3.65h-2.3z"/></svg>
            </svg>
        }
    }
}


