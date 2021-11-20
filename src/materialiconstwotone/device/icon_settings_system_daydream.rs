
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M9 15h6.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5h-.87l-.17-.86C14.29 9.92 13.23 9 12 9c-.96 0-1.84.57-2.26 1.45l-.27.57h-.73C7.74 11.15 7 11.99 7 13c0 1.1.9 2 2 2z" opacity=".3"/><path d="M9 17h6.5c1.93 0 3.5-1.57 3.5-3.5 0-1.66-1.16-3.05-2.74-3.41C15.66 8.28 13.95 7 12 7c-1.53 0-2.96.8-3.78 2.08C6.36 9.44 5 11.07 5 13c0 2.21 1.79 4 4 4zm-.26-5.98h.74l.27-.57C10.16 9.57 11.04 9 12 9c1.23 0 2.29.92 2.46 2.14l.17.86h.87c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5H9c-1.1 0-2-.9-2-2 0-1.01.74-1.85 1.74-1.98zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z"/></svg>
            </svg>
        }
    }
}


