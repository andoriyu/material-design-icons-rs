
pub struct IconStroller {
  props: crate::Props,
}

impl yew::Component for IconStroller {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><g><circle cx="16" cy="20" r="2"/><circle cx="6" cy="20" r="2"/></g><path d="M22,7V6.48C22,4.56,20.52,3,18.65,3c-1.66,0-2.54,1.27-3.18,2.03l-8.8,10.32C6.12,16,6.58,17,7.43,17L15,17 c1.1,0,2-0.9,2-2V6.27C17.58,5.59,17.97,5,18.65,5C19.42,5,20,5.66,20,6.48V7H22z"/><path d="M14.3,4.1C13.03,3.4,11.56,3,10,3C8.03,3,6.21,3.64,4.72,4.72l4.89,4.89L14.3,4.1z"/></g></svg>
            </svg>
        }
    }
}


