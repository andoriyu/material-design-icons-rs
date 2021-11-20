
pub struct IconEggAlt {
  props: crate::Props,
}

impl yew::Component for IconEggAlt {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M19,9C17,7,15.99,2,9.97,2C4.95,2,1.94,6,2,11.52C2.06,17.04,6.96,19,9.97,19c2.01,0,2.01,3,6.02,3C19,22,22,19,22,15.02 C22,12,21.01,11,19,9z M15.99,20c-1.49,0-1.96-0.5-2.68-1.26C12.66,18.05,11.66,17,9.97,17C8.33,17,4.05,16.01,4,11.5 C3.97,8.99,4.68,6.88,5.99,5.55C7.01,4.52,8.35,4,9.97,4c3.34,0,4.51,1.86,5.86,4.02c0.55,0.88,1.07,1.71,1.76,2.39 c1.9,1.89,2.41,2.4,2.41,4.61C20,17.87,17.88,20,15.99,20z"/></g><g><circle cx="12" cy="12" r="3.5"/></g></g></g></svg>
            </svg>
        }
    }
}


