
pub struct IconSettingsSystemDaydream {
  props: crate::Props,
}

impl yew::Component for IconSettingsSystemDaydream {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M15.5 17H9c-2.21 0-4-1.79-4-4 0-1.93 1.36-3.56 3.22-3.92C9.04 7.8 10.47 7 12 7c1.95 0 3.66 1.28 4.26 3.09 1.58.36 2.74 1.75 2.74 3.41 0 1.93-1.57 3.5-3.5 3.5zm-6.76-5.98C7.74 11.15 7 11.99 7 13c0 1.1.9 2 2 2h6.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5h-.87l-.17-.86C14.29 9.92 13.23 9 12 9c-.96 0-1.84.57-2.26 1.45l-.27.57h-.73zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z"/></svg>
            </svg>
        }
    }
}


