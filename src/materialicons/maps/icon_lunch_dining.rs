
pub struct IconLunchDining {
  props: crate::Props,
}

impl yew::Component for IconLunchDining {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M22,10c0.32-3.28-4.28-6-9.99-6C6.3,4,1.7,6.72,2.02,10H22z" fill-rule="evenodd"/><path d="M5.35,13.5c0.55,0,0.78,0.14,1.15,0.36c0.45,0.27,1.07,0.64,2.18,0.64 s1.73-0.37,2.18-0.64c0.37-0.23,0.59-0.36,1.15-0.36c0.55,0,0.78,0.14,1.15,0.36c0.45,0.27,1.07,0.64,2.18,0.64 c1.11,0,1.73-0.37,2.18-0.64c0.37-0.23,0.59-0.36,1.15-0.36c0.55,0,0.78,0.14,1.15,0.36c0.45,0.27,1.07,0.63,2.17,0.64v-1.98 c0,0-0.79-0.16-1.16-0.38c-0.45-0.27-1.07-0.64-2.18-0.64c-1.11,0-1.73,0.37-2.18,0.64c-0.37,0.23-0.6,0.36-1.15,0.36 s-0.78-0.14-1.15-0.36c-0.45-0.27-1.07-0.64-2.18-0.64s-1.73,0.37-2.18,0.64c-0.37,0.23-0.59,0.36-1.15,0.36 c-0.55,0-0.78-0.14-1.15-0.36c-0.45-0.27-1.07-0.64-2.18-0.64c-1.11,0-1.73,0.37-2.18,0.64C2.78,12.37,2.56,12.5,2,12.5v2 c1.11,0,1.73-0.37,2.21-0.64C4.58,13.63,4.8,13.5,5.35,13.5z" fill-rule="evenodd"/><path d="M2,16v2c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-2H2z" fill-rule="evenodd"/></g></g></svg>
            </svg>
        }
    }
}

