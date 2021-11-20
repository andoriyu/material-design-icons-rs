
pub struct IconForest {
  props: crate::Props,
}

impl yew::Component for IconForest {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><polygon opacity=".3" points="16.48,10 18.16,10 15,5.49 13.22,8.03 16,12 14.14,12 16.71,16 20.34,16"/><polygon opacity=".3" points="12.16,10 9,5.49 5.84,10 7.52,10 3.66,16 14.34,16 10.48,10"/><path d="M20.14,12H22L15,2l-3,4.29L9,2L2,12h1.86L0,18h7v4h4v-4h2v4h4v-4h7L20.14,12z M3.66,16l3.86-6H5.84L9,5.49L12.16,10h-1.68 l3.86,6H3.66z M16.71,16l-2.57-4H16l-2.78-3.97L15,5.49L18.16,10h-1.68l3.86,6H16.71z"/></g></g></svg>
            </svg>
        }
    }
}


