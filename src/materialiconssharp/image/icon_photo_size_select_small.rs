
pub struct IconPhotoSizeSelectSmall {
  props: crate::Props,
}

impl yew::Component for IconPhotoSizeSelectSmall {
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
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M23 15h-2v2h2v-2zm0 4h-2v2h2v-2zm0-8h-2v2h2v-2zm-8-8h-2v2h2V3zm8 4h-2v2h2V7zM1 21h10v-6H1v6zM3 7H1v2h2V7zm12 12h-2v2h2v-2zm4-16h-2v2h2V3zm4 0h-2v2h2V3zm-4 16h-2v2h2v-2zM3 11H1v2h2v-2zm8-8H9v2h2V3zM7 3H5v2h2V3zM3 3H1v2h2V3z"/></svg>
            </svg>
        }
    }
}


