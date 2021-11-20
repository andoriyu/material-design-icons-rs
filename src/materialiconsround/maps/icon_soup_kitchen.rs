
pub struct IconSoupKitchen {
  props: crate::Props,
}

impl yew::Component for IconSoupKitchen {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M6.15,13.5c-0.46,0-0.8-0.42-0.71-0.87C5.48,12.45,5.5,12.24,5.5,12c0-1-1-2.85-1-3.62c0-0.29,0.03-0.59,0.17-0.93 C4.78,7.18,5.04,7,5.34,7c0.45,0,0.8,0.42,0.71,0.86C6.01,8.04,6,8.21,6,8.38C6,9.15,7,11,7,12c0,0.42-0.08,0.76-0.17,1.01 C6.73,13.31,6.46,13.5,6.15,13.5z M12.65,13.5c0.31,0,0.58-0.19,0.68-0.49c0.09-0.25,0.17-0.59,0.17-1.01c0-1-1-2.85-1-3.62 c0-0.17,0.01-0.34,0.04-0.51C12.63,7.42,12.29,7,11.84,7c-0.29,0-0.56,0.18-0.67,0.45C11.03,7.79,11,8.08,11,8.38 C11,9.15,12,11,12,12c0,0.24-0.02,0.45-0.06,0.63C11.85,13.08,12.19,13.5,12.65,13.5z M9.4,13.5c0.31,0,0.58-0.19,0.68-0.49 c0.09-0.25,0.17-0.59,0.17-1.01c0-1-1-2.85-1-3.62c0-0.17,0.01-0.34,0.04-0.51C9.38,7.42,9.04,7,8.59,7C8.29,7,8.03,7.18,7.92,7.45 C7.78,7.79,7.75,8.08,7.75,8.38c0,0.77,1,2.63,1,3.62c0,0.24-0.02,0.45-0.06,0.63C8.6,13.08,8.94,13.5,9.4,13.5z M20.46,6.37 c0.57,0.07,1.08-0.34,1.12-0.91C21.59,5.28,21.6,5.12,21.6,5c0-1.65-1.35-3-3-3c-1.54,0-2.81,1.16-2.98,2.65L14.53,15H3.99 c-0.6,0-1.07,0.54-0.98,1.14C3.54,19.46,6.39,22,9.75,22c3.48,0,6.34-2.73,6.71-6.23l1.15-10.87C17.66,4.39,18.08,4,18.6,4 c0.55,0,1,0.45,1,1c0,0.07-0.01,0.18-0.01,0.31C19.55,5.84,19.93,6.3,20.46,6.37L20.46,6.37z"/></svg>
            </svg>
        }
    }
}


