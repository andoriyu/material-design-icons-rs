
pub struct IconHeadsetOff {
  props: crate::Props,
}

impl yew::Component for IconHeadsetOff {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><path d="M12,4c3.87,0,7,3.13,7,7v1h-4v0.17l6,6V11c0-4.97-4.03-9-9-9C9.98,2,8.12,2.67,6.62,3.8l1.43,1.43C9.17,4.45,10.53,4,12,4 z"/><path d="M2.1,2.1L0.69,3.51l3.33,3.33C3.37,8.09,3,9.5,3,11v9h6v-8H5v-1c0-0.94,0.19-1.83,0.52-2.65L15,17.83V20h2.17l1,1H12v2 h8.17l0.31,0.31l1.41-1.41L2.1,2.1z"/></g></g></svg>
            </svg>
        }
    }
}


