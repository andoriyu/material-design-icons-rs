
pub struct Icon9mp {
  props: crate::Props,
}

impl yew::Component for Icon9mp {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><rect height="1.5" opacity=".3" width="1.5" x="15" y="14"/><rect height="1.5" opacity=".3" width="1.5" x="11.5" y="6.5"/><path d="M5,19h14V5H5V19z M18,13.5V16c0,0.55-0.45,1-1,1h-2v1.5h-1.5v-6H17C17.55,12.5,18,12.95,18,13.5z M10,10h3 V9h-2c-0.55,0-1-0.45-1-1V6.5c0-0.55,0.45-1,1-1h2.5c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1H10V10z M6,13.5c0-0.55,0.45-1,1-1h4.5 c0.55,0,1,0.45,1,1v5H11V14h-1v3H8.5v-3h-1v4.5H6V13.5z" opacity=".3"/><path d="M7.5,14h1v3H10v-3h1v4.5h1.5v-5c0-0.55-0.45-1-1-1H7c-0.55,0-1,0.45-1,1v5h1.5V14z"/><path d="M13.5,18.5H15V17h2c0.55,0,1-0.45,1-1v-2.5c0-0.55-0.45-1-1-1h-3.5V18.5z M15,14h1.5v1.5H15V14z"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M14.5,10.5v-4c0-0.55-0.45-1-1-1H11c-0.55,0-1,0.45-1,1V8c0,0.55,0.45,1,1,1h2v1h-3v1.5h3.5 C14.05,11.5,14.5,11.05,14.5,10.5z M13,8h-1.5V6.5H13V8z"/></g></g></svg>
            </svg>
        }
    }
}

