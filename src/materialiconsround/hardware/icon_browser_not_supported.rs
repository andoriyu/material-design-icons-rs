
pub struct IconBrowserNotSupported {
  props: crate::Props,
}

impl yew::Component for IconBrowserNotSupported {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M19,6v10.5l1.95,1.95C20.98,18.3,21,18.15,21,18V6c0-1.1-0.9-2-2-2H6.5l2,2H19z"/><path d="M3.86,3.95c-0.35-0.35-0.92-0.35-1.27,0c-0.35,0.35-0.35,0.92,0,1.27L3,5.64L3,18c0,1.1,0.9,2,2,2h12.36l1.42,1.42 c0.35,0.35,0.92,0.35,1.27,0c0.35-0.35,0.35-0.92,0-1.27L3.86,3.95z M5,18V7.64L15.36,18H5z"/></g></g></svg>
            </svg>
        }
    }
}


