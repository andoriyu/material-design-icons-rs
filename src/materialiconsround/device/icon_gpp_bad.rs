
pub struct IconGppBad {
  props: crate::Props,
}

impl yew::Component for IconGppBad {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M18.7,4.51l-6-2.25c-0.45-0.17-0.95-0.17-1.4,0l-6,2.25C4.52,4.81,4,5.55,4,6.39v4.7c0,4.94,3.27,9.57,7.71,10.83 c0.19,0.05,0.39,0.05,0.57,0C16.73,20.66,20,16.03,20,11.09v-4.7C20,5.55,19.48,4.81,18.7,4.51z M14.8,14.79L14.8,14.79 c-0.39,0.39-1.02,0.39-1.41,0.01L12,13.42l-1.39,1.38c-0.39,0.39-1.02,0.39-1.41,0l0,0c-0.39-0.39-0.39-1.02,0-1.41L10.59,12 L9.2,10.61c-0.39-0.39-0.39-1.02,0-1.41c0.39-0.39,1.02-0.39,1.41,0L12,10.59l1.39-1.39c0.39-0.39,1.02-0.39,1.41,0l0,0 c0.39,0.39,0.39,1.02,0,1.41L13.42,12l1.38,1.38C15.19,13.77,15.19,14.4,14.8,14.79z"/></g></svg>
            </svg>
        }
    }
}


