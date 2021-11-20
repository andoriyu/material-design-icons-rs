
pub struct IconCurrencyRuble {
  props: crate::Props,
}

impl yew::Component for IconCurrencyRuble {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M8,21c0.55,0,1-0.45,1-1v-2h3c0.55,0,1-0.45,1-1s-0.45-1-1-1H9v-2h4.5c3.22,0,5.79-2.76,5.47-6.04 C18.7,5.1,16.14,3,13.26,3l-0.96,0H8C7.45,3,7,3.45,7,4v8H6c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h1v2H6c-0.55,0-1,0.45-1,1 s0.45,1,1,1h1v2C7,20.55,7.45,21,8,21z M13.5,12H9V5h4.5C15.43,5,17,6.57,17,8.5S15.43,12,13.5,12z"/></g></svg>
            </svg>
        }
    }
}


