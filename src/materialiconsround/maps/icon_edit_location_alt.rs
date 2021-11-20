
pub struct IconEditLocationAlt {
  props: crate::Props,
}

impl yew::Component for IconEditLocationAlt {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><g><path d="M13.54,13H10c-0.55,0-1-0.45-1-1V8.46C9,8.2,9.11,7.94,9.29,7.76l5.32-5.32C13.78,2.16,12.9,2,12,2c-4.2,0-8,3.22-8,8.2 c0,3.18,2.44,6.92,7.33,11.22c0.38,0.33,0.96,0.33,1.34,0C17.56,17.12,20,13.37,20,10.2c0-1.01-0.16-1.94-0.45-2.8l-5.31,5.31 C14.06,12.89,13.8,13,13.54,13z"/></g><polygon points="11,11 13.12,11 19.28,4.84 17.16,2.72 11,8.88"/><path d="M20.71,2L20,1.29c-0.39-0.39-1.02-0.39-1.41,0l-0.72,0.72l2.12,2.12l0.72-0.72C21.1,3.02,21.1,2.39,20.71,2z"/></g></g></svg>
            </svg>
        }
    }
}


