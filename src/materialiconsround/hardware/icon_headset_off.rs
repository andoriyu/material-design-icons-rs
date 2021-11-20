
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24" y="0"/></g><g><g><path d="M12,4c3.87,0,7,3.13,7,7v1h-2c-0.6,0-1.13,0.27-1.49,0.68L21,18.17V11c0-4.97-4.03-9-9-9C9.98,2,8.12,2.67,6.62,3.8 l1.43,1.43C9.17,4.45,10.53,4,12,4z"/><path d="M21.19,21.19L2.81,2.81c-0.39-0.39-1.02-0.39-1.41,0C1,3.2,1,3.83,1.39,4.22l2.63,2.63C3.37,8.09,3,9.5,3,11v7 c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2v-4c0-1.1-0.9-2-2-2H5v-1c0-0.94,0.19-1.83,0.52-2.65L15,17.83V18c0,1.1,0.9,2,2,2h0.17l1,1H13 c-0.55,0-1,0.45-1,1s0.45,1,1,1h6c0.36,0,0.68-0.1,0.97-0.26c0.38,0.23,0.89,0.2,1.22-0.13C21.58,22.22,21.58,21.58,21.19,21.19z"/></g></g></svg>
            </svg>
        }
    }
}

