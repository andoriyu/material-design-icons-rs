
pub struct IconCampaign {
  props: crate::Props,
}

impl yew::Component for IconCampaign {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><path d="M18,12L18,12c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2C18.45,11,18,11.45,18,12z"/><path d="M16.59,16.82c-0.33,0.44-0.24,1.05,0.2,1.37c0.53,0.39,1.09,0.81,1.62,1.21c0.44,0.33,1.06,0.24,1.38-0.2 c0-0.01,0.01-0.01,0.01-0.02c0.33-0.44,0.24-1.06-0.2-1.38c-0.53-0.4-1.09-0.82-1.61-1.21c-0.44-0.33-1.06-0.23-1.39,0.21 C16.6,16.81,16.59,16.82,16.59,16.82z"/><path d="M19.81,4.81c0-0.01-0.01-0.01-0.01-0.02c-0.33-0.44-0.95-0.53-1.38-0.2c-0.53,0.4-1.1,0.82-1.62,1.22 c-0.44,0.33-0.52,0.95-0.19,1.38c0,0.01,0.01,0.01,0.01,0.02c0.33,0.44,0.94,0.53,1.38,0.2c0.53-0.39,1.09-0.82,1.62-1.22 C20.05,5.87,20.13,5.25,19.81,4.81z"/><path d="M8,9H4c-1.1,0-2,0.9-2,2v2c0,1.1,0.9,2,2,2h1v3c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-3h1l5,3V6L8,9z"/><path d="M15.5,12c0-1.33-0.58-2.53-1.5-3.35v6.69C14.92,14.53,15.5,13.33,15.5,12z"/></svg>
            </svg>
        }
    }
}


