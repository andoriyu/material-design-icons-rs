
pub struct IconMicExternalOn {
  props: crate::Props,
}

impl yew::Component for IconMicExternalOn {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M9.22,7H4.78C4.3,6.47,4,5.77,4,5c0-1.66,1.34-3,3-3s3,1.34,3,3C10,5.77,9.7,6.47,9.22,7z M16.24,2.01 C18.32,2.13,20,4.08,20,6.16L20,21c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1l0-14.91c0-0.96-0.64-1.86-1.58-2.05 C15.14,3.78,14,4.76,14,6l0,11.84c0,2.08-1.68,4.03-3.76,4.15C7.92,22.13,6,20.29,6,18H5.45c-0.26,0-0.47-0.19-0.5-0.45L4.11,9.1 C4.05,8.51,4.51,8,5.1,8H8.9c0.59,0,1.05,0.51,1,1.1l-0.85,8.45C9.02,17.81,8.8,18,8.55,18H8c0,1.24,1.14,2.22,2.42,1.96 c0.94-0.19,1.58-1.09,1.58-2.05L12,6C12,3.71,13.92,1.87,16.24,2.01z"/></g></g></svg>
            </svg>
        }
    }
}

