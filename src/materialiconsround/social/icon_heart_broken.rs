
pub struct IconHeartBroken {
  props: crate::Props,
}

impl yew::Component for IconHeartBroken {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><path d="M19.57,3.95c-1.92-1.29-4.08-1.17-5.8-0.26L12,9h1.66c0.67,0,1.15,0.65,0.96,1.29l-1.82,6.07c-0.09,0.29-0.52,0.2-0.49-0.1 L13,10h-1.67c-0.66,0-1.14-0.64-0.96-1.27l1.18-4.11c0,0,0,0,0,0C9.7,2.89,6.71,2.32,4.27,4.04C2.82,5.07,2,6.7,2,8.49 c-0.01,3.81,3.53,6.71,8.66,11.3c0.76,0.68,1.92,0.69,2.69,0.01c4.98-4.42,8.87-7.58,8.64-11.62C21.9,6.45,21,4.92,19.57,3.95z"/></g></svg>
            </svg>
        }
    }
}


