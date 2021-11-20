
pub struct IconSouthAmerica {
  props: crate::Props,
}

impl yew::Component for IconSouthAmerica {
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
            <svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><rect fill="none" height="24" width="24"/><path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M4,12c0-1.95,0.7-3.74,1.87-5.13L9,10 v1c0,1.1,0.9,2,2,2v5.59c0,0.27,0.11,0.52,0.29,0.71L12,20C7.58,20,4,16.42,4,12z M13,19.94V18l3.75-5.62 c0.16-0.25,0.25-0.54,0.25-0.83V10.5c0-0.55-0.45-1-1-1h-1.5l-1.4-1.75C12.72,7.28,12.15,7,11.54,7H8V5.07C9.18,4.39,10.54,4,12,4 c4.41,0,8,3.59,8,8C20,16.07,16.94,19.44,13,19.94z"/></svg>
            </svg>
        }
    }
}


