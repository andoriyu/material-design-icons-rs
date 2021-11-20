
pub struct IconPersonPinCircle {
  props: crate::Props,
}

impl yew::Component for IconPersonPinCircle {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M12,2c-4.2,0-8,3.22-8,8.2c0,3.18,2.45,6.92,7.34,11.22c0.36,0.32,0.97,0.32,1.33,0C17.55,17.12,20,13.38,20,10.2 C20,5.22,16.2,2,12,2z M7.69,12.49C8.88,11.56,10.37,11,12,11s3.12,0.56,4.31,1.49C15.45,13.98,13.85,15,12,15 S8.55,13.98,7.69,12.49z M12,6c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2s-2-0.9-2-2C10,6.9,10.9,6,12,6z"/></g></svg>
            </svg>
        }
    }
}


