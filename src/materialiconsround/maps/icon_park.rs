
pub struct IconPark {
  props: crate::Props,
}

impl yew::Component for IconPark {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M16.96,12h0.08c0.81,0,1.28-0.91,0.82-1.57l-5.08-7.25c-0.4-0.57-1.24-0.57-1.64,0L6.1,10.43C5.64,11.09,6.12,12,6.93,12 h0.04l-2.9,4.46C3.63,17.12,4.11,18,4.91,18h5.08v2.02c0,1.09,0.89,1.98,1.98,1.98h0c1.09,0,1.98-0.89,1.98-1.98V18h5.15 c0.8,0,1.28-0.89,0.83-1.55L16.96,12z"/></g></g></g></svg>
            </svg>
        }
    }
}


