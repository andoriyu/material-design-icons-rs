
pub struct IconWifiCalling {
  props: crate::Props,
}

impl yew::Component for IconWifiCalling {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/><rect fill="none" height="24" width="24"/></g><g><g><path d="M22,4.95C21.79,4.78,19.67,3,16.5,3c-3.18,0-5.29,1.78-5.5,1.95L16.5,12L22,4.95z"/><path d="M19.2,15.28l-2.54-0.29c-0.61-0.07-1.21,0.14-1.64,0.57l-1.84,1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85 c0.43-0.43,0.64-1.04,0.57-1.64L8.72,4.8C8.6,3.79,7.75,3.03,6.73,3.03H5c-1.13,0-2.07,0.94-2,2.07 C3.53,13.64,10.36,20.47,18.9,21c1.13,0.07,2.07-0.87,2.07-2v-1.73C20.97,16.25,20.21,15.4,19.2,15.28z"/></g></g></svg>
            </svg>
        }
    }
}


