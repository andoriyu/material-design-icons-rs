
pub struct IconHub {
  props: crate::Props,
}

impl yew::Component for IconHub {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M8.4,18.2C8.78,18.7,9,19.32,9,20c0,1.66-1.34,3-3,3s-3-1.34-3-3s1.34-3,3-3c0.44,0,0.85,0.09,1.23,0.26l1.41-1.77 c-0.92-1.03-1.29-2.39-1.09-3.69l-2.03-0.68C4.98,11.95,4.06,12.5,3,12.5c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 c0,0.07,0,0.14-0.01,0.21l2.03,0.68c0.64-1.21,1.82-2.09,3.22-2.32l0-2.16C9.96,5.57,9,4.4,9,3c0-1.66,1.34-3,3-3s3,1.34,3,3 c0,1.4-0.96,2.57-2.25,2.91v2.16c1.4,0.23,2.58,1.11,3.22,2.32l2.03-0.68C18,9.64,18,9.57,18,9.5c0-1.66,1.34-3,3-3s3,1.34,3,3 s-1.34,3-3,3c-1.06,0-1.98-0.55-2.52-1.37l-2.03,0.68c0.2,1.29-0.16,2.65-1.09,3.69l1.41,1.77C17.15,17.09,17.56,17,18,17 c1.66,0,3,1.34,3,3s-1.34,3-3,3s-3-1.34-3-3c0-0.68,0.22-1.3,0.6-1.8l-1.41-1.77c-1.35,0.75-3.01,0.76-4.37,0L8.4,18.2z"/></svg>
            </svg>
        }
    }
}


