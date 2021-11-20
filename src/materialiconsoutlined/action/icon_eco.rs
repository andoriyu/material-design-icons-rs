
pub struct IconEco {
  props: crate::Props,
}

impl yew::Component for IconEco {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><path d="M6.05,8.05c-2.73,2.73-2.73,7.17,0,9.9C7.42,19.32,9.21,20,11,20s3.58-0.68,4.95-2.05C19.43,14.47,20,4,20,4 S9.53,4.57,6.05,8.05z M14.54,16.54C13.59,17.48,12.34,18,11,18c-0.89,0-1.73-0.25-2.48-0.68c0.92-2.88,2.62-5.41,4.88-7.32 c-2.63,1.36-4.84,3.46-6.37,6c-1.48-1.96-1.35-4.75,0.44-6.54C9.21,7.72,14.04,6.65,17.8,6.2C17.35,9.96,16.28,14.79,14.54,16.54z"/></g></svg>
            </svg>
        }
    }
}


