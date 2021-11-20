
pub struct IconLan {
  props: crate::Props,
}

impl yew::Component for IconLan {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M15,22h4c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h-1v-2c0-1.1-0.9-2-2-2h-3V9h1c1.1,0,2-0.9,2-2V4c0-1.1-0.9-2-2-2h-4 C8.9,2,8,2.9,8,4v3c0,1.1,0.9,2,2,2h1v2H8c-1.1,0-2,0.9-2,2v2H5c-1.1,0-2,0.9-2,2v3c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2v-3 c0-1.1-0.9-2-2-2H8v-2h8v2h-1c-1.1,0-2,0.9-2,2v3C13,21.1,13.9,22,15,22z"/></g></svg>
            </svg>
        }
    }
}


