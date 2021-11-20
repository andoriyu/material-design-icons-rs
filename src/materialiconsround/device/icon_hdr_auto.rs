
pub struct IconHdrAuto {
  props: crate::Props,
}

impl yew::Component for IconHdrAuto {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><g><polygon points="12.04,8.04 11.95,8.04 10.35,12.59 13.64,12.59"/><path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M15,16.41l-0.78-2.22H9.78l-0.79,2.22 C8.87,16.76,8.53,17,8.16,17h0c-0.62,0-1.05-0.62-0.83-1.2l3.34-8.88C10.88,6.37,11.4,6,12,6h0c0.59,0,1.12,0.37,1.33,0.92 l3.34,8.88c0.22,0.58-0.21,1.2-0.83,1.2h0C15.46,17,15.12,16.76,15,16.41z"/></g></g></svg>
            </svg>
        }
    }
}


