
pub struct IconPhotoSizeSelectLarge {
  props: crate::Props,
}

impl yew::Component for IconPhotoSizeSelectLarge {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M17 19h2v2h-2zM1 19c0 1.1.9 2 2 2h12V11H1v8zm4.5-3.21l1.79 2.15 2.5-3.22L13 19H3l2.5-3.21zM17 3h2v2h-2zm4 8h2v2h-2zm0 4h2v2h-2zM3 3C2 3 1 4 1 5h2V3zm18 4h2v2h-2zm-8-4h2v2h-2zm8 18c1 0 2-1 2-2h-2v2zM1 7h2v2H1zm8-4h2v2H9zM5 3h2v2H5zm16 0v2h2c0-1-1-2-2-2z"/></svg>
            </svg>
        }
    }
}


