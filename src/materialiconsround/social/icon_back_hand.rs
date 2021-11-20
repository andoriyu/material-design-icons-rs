
pub struct IconBackHand {
  props: crate::Props,
}

impl yew::Component for IconBackHand {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M21,16c0,4.42-3.58,8-8,8c-3.26,0-6.19-1.99-7.4-5.02l-3.03-7.61C2.26,10.58,3,9.79,3.81,10.05l0.79,0.26 c0.56,0.18,1.02,0.61,1.24,1.16l1.28,3.21C7.2,14.88,7.38,15,7.58,15H8V3.25C8,2.56,8.56,2,9.25,2s1.25,0.56,1.25,1.25v8.25 c0,0.28,0.22,0.5,0.5,0.5l0,0c0.28,0,0.5-0.22,0.5-0.5V1.25C11.5,0.56,12.06,0,12.75,0S14,0.56,14,1.25V11.5 c0,0.28,0.22,0.5,0.5,0.5l0,0c0.28,0,0.5-0.22,0.5-0.5V2.75c0-0.69,0.56-1.25,1.25-1.25c0.69,0,1.25,0.56,1.25,1.25v8.75 c0,0.28,0.22,0.5,0.5,0.5l0,0c0.28,0,0.5-0.22,0.5-0.5V5.75c0-0.69,0.56-1.25,1.25-1.25S21,5.06,21,5.75V16z"/></svg>
            </svg>
        }
    }
}

